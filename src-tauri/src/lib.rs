mod app;
mod commands;
mod core;
mod location;
mod orders;
mod person;
mod product;
mod warehouse;

use argon2::{self, Config};

use tauri::async_runtime::Mutex;
use tauri::Manager;

use wasm_bindgen::prelude::*;

use crate::app::AppData;
use crate::core::database;

/// WASM bindings for Tauri invoke
#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Main entry point for the application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let salt = b"somesalt";

    tauri::Builder::default()
        // Add plugins
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_stronghold::Builder::new(|pass| {
                argon2::hash_raw(pass.as_bytes(), salt, &Config::default())
                    .expect("Failed to generate hash for password")
            })
            .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        // Setup application
        .setup(|app| {
            // Initialize app state
            app.manage(Mutex::new(AppData::default()));

            // Initialize database based on platform
            let app_clone: tauri::AppHandle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = database::initialize_db(&app_clone).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });

            Ok(())
        })
        // Register command handlers
        .invoke_handler(tauri::generate_handler![
            // Product commands
            product::commands::get_products,
            product::commands::create_product,
            product::commands::get_categories,
            product::commands::create_category,
            product::commands::get_measure_units,
            product::commands::create_measure_unit,
            // Warehouse commands
            warehouse::commands::list_warehouses,
            warehouse::commands::create_warehouse,
            // Location commands
            location::commands::list_locations,
            location::commands::create_location,
            // Person commands
            person::commands::list_people,
            person::commands::create_person,
            // Order commands
            orders::commands::list_orders,
            orders::commands::create_order,
            // Other commands
            commands::set_complete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
