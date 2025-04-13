use crate::core::error::{custom_error, Result};
use crate::core::repository::DuckDbRepository;
use chrono::Utc;
use serde::Deserialize;

use super::inputs::LocationDto;
use super::model::Location;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_term: Option<String>,
}

pub fn get_location_with_filter(
    repo: &DuckDbRepository<Location>,
    filters: &FilterOptions,
) -> Result<Vec<Location>> {
    let conn = repo.get_connection()?;

    let mut query = String::from(format!(
        "
            SELECT
                {}
            FROM location l
            WHERE 1=1
        ",
        Location::get_select_for("l".to_string(), "".to_string())
    ));

    let mut params: Vec<Box<Location>> = Vec::new();

    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND l.name LIKE ?");
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern));
    }

    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        query.push_str(&format!(" ORDER BY l.{} {}", sort_by, sort_order));
    } else {
        query.push_str(" ORDER BY l.id DESC");
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

    let mut locations = Vec::new();
    while let Some(row) = rows.next()? {
        let location: Location = Location::from_row(row, &stmt)?;
        locations.push(location);
    }

    Ok(locations)
}

pub fn get_location_by_id(repo: &DuckDbRepository<Location>, id: &str) -> Result<Option<Location>> {
    let conn = repo.get_connection()?;

    let query = format!(
        "SELECT {} FROM location l WHERE l.id = ?",
        Location::get_select_for("l".to_string(), "".to_string())
    );
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let location: Location = Location::from_row(row, &stmt);
        return Ok(Some(location));
    }
    Ok(None)
}

pub fn create_location(
    repo: &DuckDbRepository<Location>,
    location_dto: &LocationDto,
) -> Result<Location> {
    let conn = repo.get_connection()?;

    // Generate new UUID
    let now = Utc::now();

    // Create the location with fields from DTO
    let location = Location {
        id: -1,
        name: location_dto.name.clone(),
        created_at: Some(now),
        updated_at: Some(now),
    };

    // Insert into the database
    let (insert_sql, data) = location.get_insert_query();
    if insert_sql.len() > 0 {
        let mut stmt = conn.prepare(&insert_sql)?;
        stmt.insert(&data)?;

        return match stmt.raw_query().next()? {
            Some(row) => {
                let id = row.get(stmt.column_index("id")?)?;
                location.id = id;
                Ok(location)
            }
            None => Err(custom_error("failed to insert at 'location'!")),
        };
        Err(custom_error(
            "Failed to get insert query for Location model!",
        ))
    }
}

pub fn update_location(
    repo: &DuckDbRepository<Location>,
    id: &str,
    location_dto: &LocationDto,
) -> Result<Location> {
    let conn = repo.get_connection()?;

    // Check if location exists
    let location_check = get_location_by_id(repo, id)?;
    if location_check.is_none() {
        return Err(custom_error(format!("Location with id {} not found", id)));
    }

    let mut location = location_check.unwrap();
    let now = Utc::now();

    // Update fields
    location.name = location_dto.name.clone();
    location.updated_at = Some(now);

    // Update in the database
    let (update_sql, data) = location.get_update_query();
    if update_sql.len() > 0 {
        let mut stmt = conn.prepare(&update_sql)?;
        stmt.update(&data)?;
        Ok(location)
    }
    Err(custom_error(
        "Failed to get update query for Location model!",
    ))
}
