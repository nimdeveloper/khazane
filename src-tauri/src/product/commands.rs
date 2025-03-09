use std::{future::IntoFuture, sync::Mutex};

use serde::Deserialize;
use surrealdb::Error::{self, Db};
use tauri::State;

use crate::app::AppData;

use super::model::Product;

#[derive(Deserialize)]
struct FilterOptions {}

#[tauri::command]
pub async fn get_products(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Product>, Error> {
    let app_data = state.lock().unwrap();
    if let Some(db) = &app_data.db {
        if let Ok(products) = db.select::<Vec<Product>>("products").await {
            dbg!(&products);
            return Ok(products);
        }
    }
    Err(Error::from(Db(surrealdb::error::Db::Unreachable(
        "DB not initialized!!".to_string(),
    ))))
}
