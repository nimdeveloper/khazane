use crate::core::error::Result;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{inputs::OrderDto, model::Order, query::{self, FilterOptions}};

#[tauri::command]
pub async fn list_orders(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Order>> {
    let repo = repository::get_repository::<Order>(&state).await?;
    match query::get_order_with_filter(&repo, &filters) {
        Ok(orders) => Ok(orders),
        Err(e) => {
            eprintln!("Error in get_order_with_filter: {}", e);
            // Fallback to the original implementation
            repo.find_all().await
        }
    }
}

#[tauri::command]
pub async fn get_order_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<Order>> {
    let repo = repository::get_repository::<Order>(&state).await?;
    match query::get_order_by_id(&repo, &id) {
        Ok(order) => Ok(order),
        Err(e) => {
            eprintln!("Error in get_order_by_id: {}", e);
            repo.find_by_id(&id).await
        }
    }
}

#[tauri::command]
pub async fn create_order(state: State<'_, Mutex<AppData>>, order: OrderDto) -> Result<Order> {
    let repo = repository::get_repository::<Order>(&state).await?;
    match query::create_order(&repo, &order) {
        Ok(order) => Ok(order),
        Err(e) => {
            eprintln!("Error in create_order: {}", e);
            repo.create(order).await
        }
    }
}

#[tauri::command]
pub async fn update_order(
    state: State<'_, Mutex<AppData>>,
    id: String,
    order: OrderDto,
) -> Result<Order> {
    let repo = repository::get_repository::<Order>(&state).await?;
    match query::update_order(&repo, &id, &order) {
        Ok(order) => Ok(order),
        Err(e) => {
            eprintln!("Error in update_order: {}", e);
            repo.update(&id, order).await
        }
    }
}
