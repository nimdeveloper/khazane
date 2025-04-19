mod app;
mod commands;
mod core;
mod location;
// mod orders;
mod person;
mod product;
mod warehouse;

use argon2::{self, Config};

use tauri::async_runtime::Mutex;
use tauri::Manager;

use wasm_bindgen::prelude::*;

use crate::app::AppData;
use tauri::async_runtime;

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

// Initialize all migrations from each module
async fn init_migrations() {
    // Register migrations from each module
    location::migration::register_migrations().await;
    person::migration::register_migrations().await;
    warehouse::migration::register_migrations().await;
    product::migration::register_migrations().await;
    // orders::migration::register_migrations().await;

    println!("All migrations registered successfully");
}

pub async fn init(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the database
    let db_name = "khazane.db";

    // Initialize the database with the default db name
    match core::database::initialize_db(app, db_name).await {
        Ok(db_path) => {
            // Store the path in app state
            let state = app.state::<async_runtime::Mutex<AppData>>();
            let mut app_data = state.lock().await;
            app_data.db_path = Some(db_path.clone());
            drop(app_data);

            // Register all migrations
            init_migrations().await;

            // Run migrations
            if let Err(e) = core::migration::run_migrations(&state).await {
                eprintln!("Error running migrations: {}", e);
                return Err(Box::new(e));
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            Err(Box::new(e))
        }
    }
}

/// Main application entry point
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

            // Use a non-blocking task to init database and migrations
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = init(&app_handle).await {
                    eprintln!("Initialization error: {}", e);
                }
            });

            Ok(())
        })
        // Register command handlers
        .invoke_handler(tauri::generate_handler![
            // Product commands
            product::commands::get_products,
            product::commands::get_product_by_id,
            product::commands::add_product,
            product::commands::update_product,
            // Product   => MeasurementUnit
            product::commands::get_measure_units,
            product::commands::get_measure_unit_by_id,
            product::commands::create_measure_unit,
            product::commands::update_measure_unit,
            // Product   => Category
            product::commands::get_category_by_id,
            product::commands::create_category,
            product::commands::update_category,
            product::commands::get_categories,
            // Warehouse commands
            warehouse::commands::list_warehouses,
            warehouse::commands::create_warehouse,
            warehouse::commands::update_warehouse,
            warehouse::commands::get_warehouse_by_id,
            // Location commands
            location::commands::list_locations,
            location::commands::create_location,
            location::commands::get_location_by_id,
            location::commands::update_location,
            // Person commands
            person::commands::list_people,
            person::commands::create_person,
            // Order commands
            // orders::commands::list_orders,
            // orders::commands::create_order,
            // Other commands
            commands::set_complete,
            commands::check_migrations,
            commands::initialize_database,
            commands::test_connection_pool,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
