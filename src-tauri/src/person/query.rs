use super::inputs::PersonDto;
use super::model::Person;
use crate::core::error::{custom_error, Result};
use crate::core::repository::{Model, Repository};
use chrono::Utc;
use duckdb::ToSql;
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
    repo: &impl Repository<Person>,
    filters: &FilterOptions,
) -> Result<Vec<Person>> {
    let conn = repo.get_connection()?;

    let mut query = String::from(format!(
        "
            SELECT
                {}
            FROM {} p
            WHERE 1=1
        ",
        Person::get_select_for("p".to_string(), "".to_string()),
        Person::TABLE_NAME
    ));

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND (p.first_name LIKE ? OR p.last_name LIKE ?)");
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern.clone()));
        params.push(Box::new(search_pattern));
    }

    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        // Map sort_by field to actual column names
        let column = match sort_by.as_str() {
            "p.full_name" => "p.first_name",
            _ => sort_by.as_str(),
        };
        query.push_str(&format!(" ORDER BY p.{} p.{}", column, sort_order));
    } else {
        query.push_str(" ORDER BY p.first_name ASC");
    }

    // Add pagination
    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.push_str(" LIMIT ? OFFSET ?");
        params.push(Box::new(limit));
        params.push(Box::new(offset));
    } else if let Some(limit) = filters.limit {
        query.push_str(" LIMIT ?");
        params.push(Box::new(limit));
    }

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(
        params
            .into_iter()
            .map(|p| p.as_ref())
            .collect::<Vec<_>>()
            .as_slice(),
    )?;

    let mut people = Vec::new();
    while let Some(row) = rows.next()? {
        let person = Person::from_row(row, stmt);
        people.push(person);
    }
    Ok(people)
}

pub fn get_person_by_id(repo: &impl Repository<Person>, id: &str) -> Result<Option<Person>> {
    let conn = repo.get_connection()?;

    let mut query = String::from(format!(
        "
            SELECT
                {}
            FROM {} p
            WHERE p.id = ?
        ",
        Person::get_select_for("p".to_string(), "".to_string()),
        Person::TABLE_NAME
    ));

    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let person = Person::from_row(row, stmt);
        return Ok(Some(person));
    }

    Ok(None)
}

pub fn create_person(repo: &impl Repository<Person>, person_dto: &PersonDto) -> Result<Person> {
    let conn = repo.get_connection()?;

    // Generate new UUID
    let now = Utc::now();

    // Create the person with fields from DTO
    let person = Person {
        id: -1,
        first_name: person_dto.first_name.clone(),
        last_name: person_dto.last_name.clone(),
        national_code: person_dto.national_code.clone(),
        phone: person_dto.phone.clone(),
        email: person_dto.email.clone(),
        address: person_dto.address.clone(),
        created_at: Some(now),
        updated_at: Some(now),
    };

    // Insert into the database
    let (insert_sql, data) = person.get_insert_query();
    if insert_sql.len() > 0 {
        let mut stmt = conn.prepare(&insert_sql)?;
        stmt.insert(&data)?;

        return match stmt.raw_query().next()? {
            Some(row) => {
                let id = row.get(stmt.column_index("id")?)?;
                person.id = id;
                Ok(person)
            }
            None => Err(custom_error("failed to insert at 'person'!")),
        };
        Err(custom_error("Failed to get insert query for Person model!"))
    }
}

pub fn update_person(
    repo: &impl Repository<Person>,
    id: &str,
    person_dto: &PersonDto,
) -> Result<Person> {
    let conn = repo.get_connection()?;

    // Check if person exists
    let person_check = get_person_by_id(repo, id)?;
    if person_check.is_none() {
        return Err(custom_error(format!("Location with id {} not found", id)));
    }

    let mut person = person_check.unwrap();
    let now = Utc::now();

    // Update fields
    person.first_name = person_dto.first_name.clone();
    person.last_name = person_dto.last_name.clone();
    person.national_code = person_dto.national_code.clone();
    person.phone = person_dto.phone.clone();
    person.email = person_dto.email.clone();
    person.address = person_dto.address.clone();
    person.updated_at = Some(now);

    // Update in the database
    let (update_sql, data) = person.get_update_query();
    if update_sql.len() > 0 {
        let mut stmt = conn.prepare(&update_sql)?;
        stmt.update(&data)?;
        Ok(person)
    }
    Err(custom_error(
        "Failed to get update query for Location model!",
    ))
}
