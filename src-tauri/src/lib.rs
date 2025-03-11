mod app;
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

use crate::app::AppData;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // .plugin(
    //     tauri_plugin_sql::Builder::new()
    //         .add_migrations("postgres://user:pass@localhost/app_db", all_migrations)
    //         .build(),
    // )
}
