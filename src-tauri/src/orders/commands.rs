use crate::core::error::Result;
use serde::Deserialize;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{inputs::OrderDto, model::Order};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_orders(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<Order>> {
    let repo = repository::get_repository::<Order>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn create_order(state: State<'_, Mutex<AppData>>, order: OrderDto) -> Result<Order> {
    let repo = repository::get_repository::<Order>(&state).await?;
    repo.create(order).await
}
