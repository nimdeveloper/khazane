use serde::Deserialize;
use surrealdb::Error;
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
    filters: FilterOptions,
) -> Result<Vec<Order>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Order>::new("order");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_order(
    state: State<'_, Mutex<AppData>>,
    order: OrderDto,
) -> Result<Option<Order>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Order>::new("order");
    repository.create(&db, order).await
}
