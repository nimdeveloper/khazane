mod app;
mod commands;
mod core;
mod location;
mod orders;
mod person;
mod product;
mod warehouse;

use argon2::{self, Config};

use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

use tauri::async_runtime::Mutex;
use tauri::Manager;

use wasm_bindgen::prelude::*;

use crate::app::AppData;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let salt = b"somesalt";
    tauri::Builder::default()
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
        .setup(|app| {
            app.manage(Mutex::new(AppData::default()));
            let dir = app
                .path()
                .app_data_dir()
                .expect("couldn't resolve app data dir")
                .join("db");
            // let scope = app.fs_scope();
            if let Some(to_resolve_db_path) = dir.to_str() {
                let db_path = to_resolve_db_path.to_owned();
                let handle = app.handle().to_owned();
                tauri::async_runtime::spawn(async move {
                    let data = Surreal::new::<RocksDb>(db_path).await;
                    if let Ok(db) = data {
                        db.use_ns("khazane").use_db("db").await.unwrap();
                        let state = handle.state::<Mutex<AppData>>();
                        let mut state = state.lock().await;
                        state.db = Some(db.to_owned());
                    }
                });
            }
            // app.manage();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            product::commands::get_products,
            product::commands::add_product,
            product::commands::get_categories,
            product::commands::add_category,
            warehouse::commands::list_warehouses,
            warehouse::commands::create_warehouse,
            location::commands::list_locations,
            location::commands::create_location,
            person::commands::list_people,
            person::commands::create_person,
            orders::commands::list_orders,
            orders::commands::create_order,
            commands::set_complete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // .plugin(
    //     tauri_plugin_sql::Builder::new()
    //         .add_migrations("postgres://user:pass@localhost/app_db", all_migrations)
    //         .build(),
    // )
}
