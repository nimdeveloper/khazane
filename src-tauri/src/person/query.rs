use crate::core::error::Result;
use crate::core::repository::Repository;
use chrono::Utc;
use serde::Deserialize;
use std::error::Error;
use uuid::Uuid;

use super::inputs::PersonDto;
use super::model::Person;

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
    let mut query = String::from("
        SELECT id, first_name, last_name, national_code, phone, email, address, created_at, updated_at
        FROM person
        WHERE 1=1
    ");

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND (first_name LIKE ? OR last_name LIKE ?)");
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern.clone()));
        params.push(Box::new(search_pattern));
    }

    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        // Map sort_by field to actual column names
        let column = match sort_by.as_str() {
            "full_name" => "first_name",
            _ => sort_by.as_str(),
        };
        query.push_str(&format!(" ORDER BY {} {}", column, sort_order));
    } else {
        query.push_str(" ORDER BY first_name ASC");
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
        let person = Person {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            national_code: row.get(3)?,
            phone: row.get(4)?,
            email: row.get(5)?,
            address: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        };
        people.push(person);
    }

    Ok(people)
}

trait ToSql {
    fn as_ref(&self) -> &dyn rusqlite::ToSql;
}

impl<T: rusqlite::ToSql + ?Sized> ToSql for T {
    fn as_ref(&self) -> &dyn rusqlite::ToSql {
        self
    }
}

pub fn get_person_by_id(
    repo: &impl Repository<Person>,
    id: &str,
) -> Result<Option<Person>, Box<dyn Error>> {
    let conn = repo.get_connection()?;

    let query = "
        SELECT id, first_name, last_name, national_code, phone, email, address, created_at, updated_at
        FROM person
        WHERE id = ?
    ";

    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let person = Person {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            national_code: row.get(3)?,
            phone: row.get(4)?,
            email: row.get(5)?,
            address: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        };
        return Ok(Some(person));
    }

    Ok(None)
}

pub fn create_person(
    repo: &impl Repository<Person>,
    person_dto: &PersonDto,
) -> Result<Person, Box<dyn Error>> {
    let conn = repo.get_connection()?;

    // Generate new UUID
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // Create the person with fields from DTO
    let person = Person {
        id: id.clone(),
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
    let insert_sql = "
        INSERT INTO person (id, first_name, last_name, national_code, phone, email, address, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    ";

    let mut stmt = conn.prepare(insert_sql)?;
    stmt.execute(&[
        &person.id,
        &person.first_name,
        &person.last_name,
        &person.national_code,
        &person.phone,
        &person.email,
        &person.address,
        &person.created_at,
        &person.updated_at,
    ])?;

    Ok(person)
}

pub fn update_person(
    repo: &impl Repository<Person>,
    id: &str,
    person_dto: &PersonDto,
) -> Result<Person, Box<dyn Error>> {
    let conn = repo.get_connection()?;

    // Check if person exists
    let person_check = get_person_by_id(repo, id)?;
    if person_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Person with id {} not found", id),
        )));
    }

    let now = Utc::now();

    // Update in the database with new values
    let update_sql = "
        UPDATE person 
        SET first_name = ?, last_name = ?, national_code = ?, phone = ?, email = ?, address = ?, updated_at = ?
        WHERE id = ?
    ";

    let mut stmt = conn.prepare(update_sql)?;
    stmt.execute(&[
        &person_dto.first_name,
        &person_dto.last_name,
        &person_dto.national_code,
        &person_dto.phone,
        &person_dto.email,
        &person_dto.address,
        &now,
        id,
    ])?;

    // Get the updated person
    let updated_person = get_person_by_id(repo, id)?.unwrap();

    Ok(updated_person)
}
