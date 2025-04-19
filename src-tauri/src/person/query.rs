#![allow(dead_code)]
use super::inputs::PersonDto;
use super::model::Person;
use crate::core::error::{custom_error, Result};
use crate::core::repository::DuckDbRepository;
use crate::core::selector::{Internal, Operations, OrderDirection};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_term: Option<String>,
}

pub fn get_person_with_filter(
    repo: &DuckDbRepository<Person>,
    filters: &FilterOptions,
) -> Result<Vec<Person>> {
    let conn = repo.get_connection()?;

    let mut query = Person::select();

    if let Some(search_term) = &filters.search_term {
        // Add search filter
        query.filter(
            Internal::Field("first_name".into()),
            Operations::Like,
            Internal::Value(format!("%{}%", search_term.to_owned()).into()),
        );
        query.filter(
            Internal::Field("p.last_name".into()),
            Operations::Like,
            Internal::Value(format!("%{}%", search_term.to_owned()).into()),
        );
    }

    // Add sorting
    if let Some(sort_by) = &filters.sort_by {
        if let Some(sort_order) = &filters.sort_order {
            query.order(sort_by.to_owned().into(), sort_order.to_owned().into());
        } else {
            query.order(sort_by.to_owned().into(), OrderDirection::DESC);
        }
    } else {
        query.order("id".into(), OrderDirection::ASC);
    }

    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.paginate(limit, Some(offset));
    } else if let Some(limit) = filters.limit {
        query.paginate(limit, None);
    }
    let mut people: Vec<Person> = Vec::new();
    let (stmt, translate) = query.all(&conn)?;
    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let person = Person::from_row(row, &translate);
        if let Ok(person) = person {
            if !(people.iter().filter(|&e| e.id == person.id).count() > 0) {
                people.push(person);
            }
        } else {
            return Err(custom_error("Failed to construct Person from row!"));
        }
    }
    Ok(people)
}

pub fn get_person_by_id(repo: &DuckDbRepository<Person>, id: &str) -> Result<Option<Person>> {
    let conn = repo.get_connection()?;

    let mut query = Person::select();
    query.filter(
        Internal::Field("id".into()),
        Operations::EqualTo,
        Internal::Value(id.to_string().into()),
    );
    let result = query.all(&conn);
    if let Ok((stmt, translate)) = result {
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let person: Person = Person::from_row(row, &translate)?;
            return Ok(Some(person));
        }
    }
    Ok(None)
}

pub fn create_person(repo: &DuckDbRepository<Person>, person_dto: &PersonDto) -> Result<Person> {
    let conn = repo.get_connection()?;

    // Create the person with fields from DTO
    let mut person = Person::new(
        person_dto.first_name.clone(),
        person_dto.last_name.clone(),
        person_dto.national_code.clone(),
        person_dto.phone.clone(),
        person_dto.email.clone(),
        person_dto.address.clone(),
    );

    // Insert into the database
    person.save(&conn)?;
    Ok(person)
}

pub fn update_person(
    repo: &DuckDbRepository<Person>,
    id: &str,
    person_dto: &PersonDto,
) -> Result<Person> {
    let conn = repo.get_connection()?;

    // Check if person exists
    let person_check = get_person_by_id(repo, id)?;
    if person_check.is_none() {
        return Err(custom_error(format!("Person with id {} not found", id)));
    }

    let mut person = person_check.unwrap();

    // Update fields
    person.first_name = person_dto.first_name.clone();
    person.last_name = person_dto.last_name.clone();
    person.national_code = person_dto.national_code.clone();
    person.phone = person_dto.phone.clone();
    person.email = person_dto.email.clone();
    person.address = person_dto.address.clone();

    // Update in the database
    person.save(&conn)?;

    Ok(person)
}
