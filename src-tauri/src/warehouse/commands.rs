use serde::Deserialize;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::{
        error::Result,
        repository::{self, Repository},
    },
};

use super::{inputs::WarehouseDto, model::Warehouse};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_warehouses(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<Warehouse>> {
    let repo = repository::get_repository::<Warehouse>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn create_warehouse(
    state: State<'_, Mutex<AppData>>,
    warehouse: WarehouseDto,
) -> Result<Warehouse> {
    let repo = repository::get_repository::<Warehouse>(&state).await?;
    repo.create(warehouse).await
}
