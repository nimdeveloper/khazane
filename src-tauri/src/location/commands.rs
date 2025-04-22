use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::{error::Result, repository},
};

use super::{
    inputs::LocationDto,
    model::Location,
    query::{self, FilterOptions},
};

#[tauri::command]
pub async fn list_locations(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Location>> {
    let repo = repository::get_repository::<Location>(&state).await?;
    match query::get_location_with_filter(&repo, &filters) {
        Ok(locations) => Ok(locations),
        Err(e) => {
            log::error!("Error in get_location_with_filter: {}", e);
            // Fallback to the original implementation
            // repo.find_all().await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn get_location_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<Location>> {
    let repo = repository::get_repository::<Location>(&state).await?;
    match query::get_location_by_id(&repo, &id) {
        Ok(location) => Ok(location),
        Err(e) => {
            log::error!("Error in get_location_by_id: {}", e);
            // repo.find_by_id(&id).await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn create_location(
    state: State<'_, Mutex<AppData>>,
    location: LocationDto,
) -> Result<Location> {
    let repo = repository::get_repository::<Location>(&state).await?;
    match query::create_location(&repo, &location) {
        Ok(location) => Ok(location),
        Err(e) => {
            log::error!("Error in create_location: {}", e);
            // repo.create(location).await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn update_location(
    state: State<'_, Mutex<AppData>>,
    id: String,
    location: LocationDto,
) -> Result<Location> {
    let repo = repository::get_repository::<Location>(&state).await?;
    match query::update_location(&repo, &id, &location) {
        Ok(location) => Ok(location),
        Err(e) => {
            log::error!("Error in update_location: {}", e);
            Err(e)
            // repo.update(&id, location).await
        }
    }
}
