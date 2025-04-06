use crate::{app::AppData, core::database, core::error::Result};
use tauri::{AppHandle, Manager};

// A custom task for setting the state of a setup task
#[tauri::command]
pub async fn set_complete(app: AppHandle) -> std::result::Result<i64, String> {
    // Get the state
    let state_handle = app.state::<tauri::async_runtime::Mutex<AppData>>();
    let state = state_handle.lock().await;

    let mut loaded = true;
    if state.db_path.is_none() {
        loaded = false;
    }

    // Check if both tasks are completed
    if loaded {
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        if let Some(splash_window) = app.get_webview_window("splashscreen") {
            let main_window = app.get_webview_window("main").unwrap();
            if let Err(e) = splash_window.close() {
                return Err(e.to_string());
            }
            if let Err(e) = main_window.show() {
                return Err(e.to_string());
            }
            return Ok(0);
        }
    }

    Ok(1)
}

#[tauri::command]
pub async fn check_migrations(app: AppHandle) -> Result<bool> {
    let state = app.state::<tauri::async_runtime::Mutex<AppData>>();
    crate::core::migration::run_migrations(&state).await
}

// Command to initialize database
#[tauri::command]
pub async fn initialize_database(app: AppHandle, db_name: String) -> Result<String> {
    let db_path = database::initialize_db(&app, &db_name).await?;

    // Update app state with db path
    let state = app.state::<tauri::async_runtime::Mutex<AppData>>();
    let mut app_data = state.lock().await;
    app_data.db_path = Some(db_path.clone());

    Ok(db_path)
}

// Test command to check our connection pool
#[tauri::command]
pub async fn test_connection_pool(app: AppHandle) -> Result<String> {
    // Get a connection from app state
    let state = app.state::<tauri::async_runtime::Mutex<AppData>>();
    let app_data = state.lock().await;
    let conn = app_data.get_connection()?;

    // Try to execute a simple query
    let mut stmt = conn.prepare("SELECT 'Connection pool is working!';")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let message: String = row.get(0)?;
        Ok(message)
    } else {
        Ok("Query executed but no results returned".to_string())
    }
}
