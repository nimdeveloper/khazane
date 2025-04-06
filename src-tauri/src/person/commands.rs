use serde::Deserialize;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::{
        error::Result,
        repository::{self, Repository},
    },
};

use super::{inputs::PersonDto, model::Person};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_people(
    state: State<'_, Mutex<AppData>>,
    _filters: FilterOptions,
) -> Result<Vec<Person>> {
    let repo = repository::get_repository::<Person>(&state).await?;
    repo.find_all().await
}

#[tauri::command]
pub async fn create_person(state: State<'_, Mutex<AppData>>, person: PersonDto) -> Result<Person> {
    let repo = repository::get_repository::<Person>(&state).await?;
    repo.create(person).await
}
