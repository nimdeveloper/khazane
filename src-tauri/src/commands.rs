use tauri::{async_runtime::Mutex, AppHandle, Manager, State};

use crate::app::AppData;

// A custom task for setting the state of a setup task
#[tauri::command]
pub async fn set_complete(app: AppHandle, state: State<'_, Mutex<AppData>>) -> Result<i64, ()> {
    let mut loaded = true;
    // Lock the state without write access
    let state_lock = state.lock().await;

    if let None = &state_lock.db {
        loaded = false;
    }

    // Check if both tasks are completed
    if loaded {
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
        return Ok(0);
    }

    Ok(1)
}
