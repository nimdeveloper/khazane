use serde::Deserialize;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::{
        error::Result,
        repository::{self, Repository},
    },
};

use super::{inputs::LocationDto, model::Location};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_locations(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<Location>> {
    let repo = repository::get_repository::<Location>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn create_location(
    state: State<'_, Mutex<AppData>>,
    location: LocationDto,
) -> Result<Location> {
    let repo = repository::get_repository::<Location>(&state).await?;
    repo.create(location).await
}
