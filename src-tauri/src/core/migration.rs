#![allow(dead_code)]

use crate::app::AppData;
use crate::core::database;
use crate::core::error::Result;
use duckdb::Connection;
use std::collections::HashMap;
use std::io;
use std::sync::Once;
use tauri::async_runtime::{Mutex, RwLock};

// Migration record structure
pub struct Migration {
    pub id: String,
    pub applied_at: chrono::NaiveDateTime,
    pub service: String,
    pub name: String,
    pub version: i32,
}

// A single migration definition
#[derive(Debug, Clone)]
pub struct MigrationDefinition {
    pub service: &'static str,
    pub name: &'static str,
    pub version: i32,
    pub up_sql: &'static str,
    pub down_sql: Option<&'static str>,
    pub dependency: Option<Vec<&'static str>>,
}

// Global migration registry
type MigrationRegistry = HashMap<String, MigrationDefinition>;
static MIGRATION_REGISTRY: std::sync::LazyLock<RwLock<Option<MigrationRegistry>>> =
    std::sync::LazyLock::new(|| RwLock::new(None));
static INIT: Once = Once::new();

fn prepare() {
    let mut registry = MIGRATION_REGISTRY.try_write().unwrap();
    *registry = Some(HashMap::new());
}
// Initialize the migration registry
fn init_registry() {
    INIT.call_once(|| {
        prepare();
    });
}

// Register a migration
pub async fn register_migration(migration: MigrationDefinition) {
    init_registry();

    let id = format!("{}.{}", migration.service, migration.version);
    let registry = MIGRATION_REGISTRY.write();

    if let Some(registry) = registry.await.as_mut() {
        registry.insert(id, migration);
    }
}

/// Create migrations table if it doesn't exist
pub fn init_migrations_table(conn: &Connection) -> Result<()> {
    // Check if migrations table exists
    if !database::table_exists(conn, "migrations")? {
        // Create migrations table
        conn.execute_batch(
            "CREATE TABLE migrations (
                id TEXT PRIMARY KEY,
                applied_at TIMESTAMP,
                service TEXT,
                name TEXT,
                version INTEGER
            )",
        )?;
    }

    Ok(())
}

/// Check if a migration has been applied
pub fn is_migration_applied(conn: &Connection, service: &str, version: i32) -> Result<bool> {
    let query = "SELECT COUNT(*) FROM migrations WHERE service = ? AND version = ?";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[service, &version.to_string()])?;

    if let Some(row) = rows.next()? {
        let count: i64 = row.get(0)?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

/// Record that a migration has been applied
pub fn record_migration(conn: &Connection, service: &str, name: &str, version: i32) -> Result<()> {
    let id = format!("{}_{}", service, version);
    let now = chrono::Utc::now();

    conn.execute(
        "INSERT INTO migrations (id, applied_at, service, name, version) VALUES (?, ?, ?, ?, ?)",
        &[&id, &now.to_rfc3339(), service, name, &version.to_string()],
    )?;

    Ok(())
}

/// Remove a migration record
pub fn remove_migration_record(conn: &Connection, service: &str, version: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM migrations WHERE service = ? AND version = ?",
        &[service, &version.to_string()],
    )?;

    Ok(())
}

/// Apply a migration if it hasn't been applied yet
pub fn apply_migration(conn: &Connection, migration: &MigrationDefinition) -> Result<bool> {
    println!(
        "Applying migration {}.{}: {}",
        migration.service, migration.version, migration.name
    );
    if !is_migration_applied(conn, migration.service, migration.version)? {
        // Apply the migration
        conn.execute_batch(migration.up_sql)?;

        // Record that the migration has been applied
        record_migration(conn, migration.service, migration.name, migration.version)?;

        println!("✅ Migration Applied",);
        Ok(true)
    } else {
        println!("⛔ Migration already applied!");
        // Migration already applied
        Ok(false)
    }
}

/// Revert a migration if it has been applied
pub fn revert_migration(conn: &Connection, migration: &MigrationDefinition) -> Result<bool> {
    if is_migration_applied(conn, migration.service, migration.version)? {
        if let Some(down_sql) = migration.down_sql {
            // Apply the down migration
            conn.execute_batch(down_sql)?;

            // Remove the migration record
            remove_migration_record(conn, migration.service, migration.version)?;

            println!(
                "Reverted migration {}.{}: {}",
                migration.service, migration.version, migration.name
            );
            Ok(true)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!(
                    "Migration {}.{}: {} cannot be reverted - no down migration defined",
                    migration.service, migration.version, migration.name
                ),
            )
            .into())
        }
    } else {
        // Migration not applied
        Ok(false)
    }
}

/// Run all migrations for all services
pub async fn run_migrations(state: &Mutex<AppData>) -> Result<bool> {
    let app_data = state.lock().await;

    // Get a connection from the manager
    let conn = app_data.get_connection()?;

    // Initialize migrations table
    init_migrations_table(&conn)?;

    // Get migrations, sort, and apply
    let mut applied_any = false;
    let migrations = get_sorted_migrations().await?;

    // Apply migrations in order
    for migration in migrations {
        applied_any |= apply_migration(&conn, &migration)?;
    }

    // Update AppData if any migrations were applied
    if applied_any {
        drop(app_data); // Release the lock to avoid deadlock
        let mut app_data = state.lock().await;
        app_data.migrations_applied = true;
    }

    Ok(applied_any)
}

// Get migrations safely
async fn get_sorted_migrations() -> Result<Vec<MigrationDefinition>> {
    // Create a thread-safe copy of migrations
    let registry_guard = MIGRATION_REGISTRY.read().await;
    let registry = registry_guard.clone(); // Clone the data to extend its lifetime
    let mut migrations: Vec<MigrationDefinition> = vec![]; // Initialize with an empty vector

    if let Some(registry) = registry {
        // Collect migrations into owned types
        migrations = registry.values().cloned().collect();

        migrations.sort_by(|a, b| a.service.cmp(b.service).then(a.version.cmp(&b.version)));
        // Sort migrations by service and version first
        migrations.sort_by(|a, b| a.service.cmp(b.service).then(a.version.cmp(&b.version)));
        // Resolve dependencies
        let mut resolved: Vec<MigrationDefinition> = vec![];
        let mut unresolved = migrations;

        while !unresolved.is_empty() {
            let mut progress = false;

            unresolved.retain(|migration| {
                if let Some(deps) = &migration.dependency {
                    if deps.iter().all(|dep| {
                        resolved
                            .iter()
                            .any(|m| format!("{}.{}", m.service, m.version) == *dep)
                    }) {
                        resolved.push(migration.clone());
                        progress = true;
                        false // Remove from unresolved
                    } else {
                        true // Keep in unresolved
                    }
                } else {
                    resolved.push(migration.clone());
                    progress = true;
                    false // Remove from unresolved
                }
            });

            if !progress {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Circular or unresolved dependency detected in migrations!",
                )
                .into());
            }
        }

        migrations = resolved;
    } else {
        // No migrations registered
    }
    Ok(migrations)
}

// Macro to register migrations
#[macro_export]
macro_rules! register_migration {
    ($service:expr, $name:expr, $version:expr, $up_sql:expr, $down_sql:expr) => {
        $crate::core::migration::register_migration($crate::core::migration::MigrationDefinition {
            service: $service,
            name: $name,
            version: $version,
            up_sql: $up_sql,
            down_sql: Some($down_sql),
            dependency: None,
        });
    };
    ($service:expr, $name:expr, $version:expr, $up_sql:expr, $down_sql:expr, $($dep:expr),+) => {
        $crate::core::migration::register_migration($crate::core::migration::MigrationDefinition {
            service: $service,
            name: $name,
            version: $version,
            up_sql: $up_sql,
            down_sql: Some($down_sql),
            dependency: Some(vec![$($dep),+]),
        })
    };
}
