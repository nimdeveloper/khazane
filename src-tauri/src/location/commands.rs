use serde::Deserialize;
use surrealdb::Error;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{inputs::LocationDto, model::Location};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_locations(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Location>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Location>::new("location");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_location(
    state: State<'_, Mutex<AppData>>,
    location: LocationDto,
) -> Result<Option<Location>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Location>::new("location");
    repository.create(&db, location).await
}
