use crate::app::AppData;
use std::path::PathBuf;
use surrealdb::{engine::local::Db, Error, Surreal};
use tauri::{async_runtime::Mutex, Manager, Runtime};

#[cfg(any(target_os = "android", target_os = "ios"))]
use surrealdb::engine::local::Mem;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use surrealdb::engine::local::RocksDb;

/// Get the appropriate database directory based on platform
fn get_db_path<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<PathBuf, Error> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| {
        Error::Api(surrealdb::error::Api::Query(
            "Failed to get app data directory".to_string(),
        ))
    })?;

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        // Desktop platform - store in app data directory
        Ok(app_data_dir.join("db"))
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile platform - store in app cache directory for better management
        let cache_dir = app.path().app_cache_dir().map_err(|_| {
            Error::Api(surrealdb::error::Api::Query(
                "Failed to get app cache directory".to_string(),
            ))
        })?;

        Ok(cache_dir.join("db_cache"))
    }
}

/// Initialize the database based on the platform
pub async fn initialize_db<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), Error> {
    let db_path = get_db_path(app)?;

    // Create directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            Error::Api(surrealdb::error::Api::Query(format!(
                "Failed to create database directory: {}",
                e
            )))
        })?;
    }

    let handle = app.clone();

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        // Desktop platform - use RocksDB
        let db_path_str = db_path.to_string_lossy().to_string();
        tauri::async_runtime::spawn(async move {
            match Surreal::new::<RocksDb>(&db_path_str).await {
                Ok(db) => {
                    if let Err(e) = db.use_ns("khazane").use_db("db").await {
                        eprintln!("Failed to use namespace and database: {}", e);
                        return;
                    }

                    let state = handle.state::<Mutex<AppData>>();
                    let mut state = state.lock().await;
                    state.db = Some(db);
                    println!("Desktop database initialized successfully");
                }
                Err(e) => {
                    eprintln!("Failed to initialize desktop database: {}", e);
                }
            }
        });
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile platform - use in-memory database for performance
        tauri::async_runtime::spawn(async move {
            match Surreal::new::<Mem>(()).await {
                Ok(db) => {
                    if let Err(e) = db.use_ns("khazane").use_db("db").await {
                        eprintln!("Failed to use namespace and database: {}", e);
                        return;
                    }

                    let state = handle.state::<Mutex<AppData>>();
                    let mut state = state.lock().await;
                    state.db = Some(db);
                    println!("Mobile database initialized successfully");
                }
                Err(e) => {
                    eprintln!("Failed to initialize mobile database: {}", e);
                }
            }
        });
    }

    Ok(())
}

/// Get the database connection from the application state
pub async fn get_db(state: &Mutex<AppData>) -> Result<Surreal<Db>, Error> {
    let app_data = state.lock().await;
    if let Some(db) = &app_data.db {
        Ok(db.clone())
    } else {
        Err(Error::Api(surrealdb::error::Api::Query(
            "Database not initialized".to_string(),
        )))
    }
}
