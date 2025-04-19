use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::{error::Result, repository},
};

use super::{
    inputs::WarehouseDto,
    model::Warehouse,
    query::{self, FilterOptions},
};

#[tauri::command]
pub async fn list_warehouses(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Warehouse>> {
    let repo = repository::get_repository::<Warehouse>(&state).await?;
    match query::get_warehouse_with_filter(&repo, &filters) {
        Ok(warehouses) => Ok(warehouses),
        Err(e) => {
            eprintln!("Error in get_warehouse_with_filter: {}", e);
            // Fallback to the original implementation
            // repo.find_all().await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn get_warehouse_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<Warehouse>> {
    let repo = repository::get_repository::<Warehouse>(&state).await?;
    match query::get_warehouse_by_id(&repo, &id) {
        Ok(warehouse) => Ok(warehouse),
        Err(e) => {
            eprintln!("Error in get_warehouse_by_id: {}", e);
            // repo.find_by_id(&id).await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn create_warehouse(
    state: State<'_, Mutex<AppData>>,
    warehouse: WarehouseDto,
) -> Result<Warehouse> {
    let repo = repository::get_repository::<Warehouse>(&state).await?;
    match query::create_warehouse(&repo, &warehouse) {
        Ok(warehouse) => Ok(warehouse),
        Err(e) => {
            eprintln!("Error in create_warehouse: {}", e);
            // repo.create(warehouse).await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn update_warehouse(
    state: State<'_, Mutex<AppData>>,
    id: String,
    warehouse: WarehouseDto,
) -> Result<Warehouse> {
    let repo = repository::get_repository::<Warehouse>(&state).await?;
    match query::update_warehouse(&repo, &id, &warehouse) {
        Ok(warehouse) => Ok(warehouse),
        Err(e) => {
            eprintln!("Error in update_warehouse: {}", e);
            // repo.update(&id, warehouse).await
            Err(e)
        }
    }
}
