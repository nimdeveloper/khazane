use serde::Deserialize;
use surrealdb::Error;
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::repository::{self, Repository},
};

use super::{inputs::PersonDto, model::Person};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {}

#[tauri::command]
pub async fn list_people(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Person>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Person>::new("person");
    repository.find_all(&db).await
}

#[tauri::command]
pub async fn create_person(
    state: State<'_, Mutex<AppData>>,
    person: PersonDto,
) -> Result<Option<Person>, Error> {
    let db = repository::get_db(&state).await?;
    let repository = Repository::<Person>::new("person");
    repository.create(&db, person).await
}
