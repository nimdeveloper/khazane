use serde::Deserialize;
use surrealdb::Error;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{inputs::WarehouseDto, model::Warehouse};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_warehouses(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Warehouse>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Warehouse>::new("warehouse");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_warehouse(
    state: State<'_, Mutex<AppData>>,
    warehouse: WarehouseDto,
) -> Result<Option<Warehouse>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Warehouse>::new("warehouse");
    repository.create(&db, warehouse).await
}
