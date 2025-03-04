// mod location;
// mod orders;
// mod person;
// mod product;
// mod warehouse;

use argon2::{self, Config};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // .plugin(
    //     tauri_plugin_sql::Builder::new()
    //         .add_migrations("postgres://user:pass@localhost/app_db", all_migrations)
    //         .build(),
    // )
}
