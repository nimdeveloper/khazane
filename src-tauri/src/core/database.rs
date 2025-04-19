#![allow(dead_code)]
use super::error::{Error, ErrorSource, Result};
use duckdb::types::{FromSql, FromSqlError, ValueRef};
use duckdb::Connection;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use tauri::{AppHandle, Manager};

// Global connection pool
type ConnectionPool = HashMap<String, Arc<Mutex<Connection>>>;
static CONNECTION_POOL: LazyLock<Mutex<ConnectionPool>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Get the appropriate database directory based on platform
pub fn get_db_path(app: &AppHandle, db_name: &str) -> Result<PathBuf> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| Error {
        source: ErrorSource::DatabaseInitialize,
        message: format!("Failed to get app data directory: {}", e),
        cause: None,
    })?;

    // Desktop platform - store in app data directory
    Ok(app_data_dir.join(db_name))
}

/// Get a connection from the pool, or create one if it doesn't exist
pub fn get_connection(db_path: &str) -> Result<Connection> {
    let pool = CONNECTION_POOL.lock().unwrap();

    if let Some(conn) = pool.get(db_path) {
        // Return a clone of the connection
        conn.lock().unwrap().try_clone().map_err(Error::from)
    } else {
        // Connection not found, create a new one
        drop(pool); // Release the lock
        create_connection(db_path)
    }
}

/// Create a new connection and add it to the pool
pub fn create_connection(db_path: &str) -> Result<Connection> {
    // Create parent directory if it doesn't exist
    let path = PathBuf::from(db_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create a new connection
    let conn = Connection::open(db_path)?;

    // Configure the connection
    conn.execute_batch(
        "
        PRAGMA enable_progress_bar=false;
        PRAGMA enable_profiling=no_output;
        PRAGMA threads=4;
        ",
    )?;

    // Clone the connection for the return value
    let return_conn = conn.try_clone()?;

    // Add the connection to the pool
    let mut pool = CONNECTION_POOL.lock().unwrap();
    pool.insert(db_path.to_string(), Arc::new(Mutex::new(conn)));

    println!("Database connection created and added to pool: {}", db_path);
    Ok(return_conn)
}

/// Initialize the database
pub async fn initialize_db(app: &AppHandle, db_name: &str) -> Result<String> {
    let db_path = get_db_path(app, db_name)?;
    let db_path_str = db_path.to_string_lossy().to_string();

    // Create a connection and add it to the pool
    create_connection(&db_path_str)?;

    println!("Database initialized successfully at {}", db_path_str);
    Ok(db_path_str)
}

/// Simple function to check if a table exists
pub fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let query = format!(
        "SELECT count(*) FROM information_schema.tables WHERE table_name = '{}'",
        table_name
    );

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let count: i64 = row.get(0)?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

/// Execute a SQL statement that doesn't return results
pub fn execute(conn: &Connection, sql: &str) -> Result<()> {
    conn.execute_batch(sql)?;
    Ok(())
}

pub fn value_ref_to_type<T: FromSql>(input: &ValueRef) -> Result<T> {
    FromSql::column_result(*input).map_err(|err| match err {
        FromSqlError::InvalidType => Error {
            source: ErrorSource::Database,
            message: "Invalid target type for converting db returned type".to_owned(),
            cause: None,
        },
        FromSqlError::OutOfRange(_) => Error {
            source: ErrorSource::Database,
            message: "Destination integer type is small! can't convert.".to_owned(),
            cause: None,
        },
        FromSqlError::Other(_) => Error {
            source: ErrorSource::Database,
            message: "Failed to convert db returned type".to_owned(),
            cause: None,
        },
        _ => Error {
            source: ErrorSource::Database,
            message: "Unknown conversion error!".to_owned(),
            cause: None,
        },
    })
}
