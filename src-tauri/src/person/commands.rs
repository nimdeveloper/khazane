#![allow(dead_code)]
use tauri::{async_runtime::Mutex, State};

use crate::{
    app::AppData,
    core::{error::Result, repository},
};

use super::{
    inputs::PersonDto,
    model::Person,
    query::{self, FilterOptions},
};

#[tauri::command]
pub async fn list_people(
    state: State<'_, Mutex<AppData>>,
    filters: FilterOptions,
) -> Result<Vec<Person>> {
    let repo = repository::get_repository::<Person>(&state).await?;
    match query::get_person_with_filter(&repo, &filters) {
        Ok(people) => Ok(people),
        Err(e) => {
            eprintln!("Error in get_person_with_filter: {}", e);
            // Fallback to the original implementation
            // repo.find_all().await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn get_person_by_id(
    state: State<'_, Mutex<AppData>>,
    id: String,
) -> Result<Option<Person>> {
    let repo = repository::get_repository::<Person>(&state).await?;
    match query::get_person_by_id(&repo, &id) {
        Ok(person) => Ok(person),
        Err(e) => {
            eprintln!("Error in get_person_by_id: {}", e);
            Err(e)
            // repo.find_by_id(&id).await
        }
    }
}

#[tauri::command]
pub async fn create_person(state: State<'_, Mutex<AppData>>, person: PersonDto) -> Result<Person> {
    let repo = repository::get_repository::<Person>(&state).await?;
    match query::create_person(&repo, &person) {
        Ok(person) => Ok(person),
        Err(e) => {
            eprintln!("Error in create_person: {}", e);
            // repo.create(person).await
            Err(e)
        }
    }
}

#[tauri::command]
pub async fn update_person(
    state: State<'_, Mutex<AppData>>,
    id: String,
    person: PersonDto,
) -> Result<Person> {
    let repo = repository::get_repository::<Person>(&state).await?;
    match query::update_person(&repo, &id, &person) {
        Ok(person) => Ok(person),
        Err(e) => {
            eprintln!("Error in update_person: {}", e);
            // repo.update(&id, person).await
            Err(e)
        }
    }
}
