use crate::core::error::{custom_error, Result};
use crate::core::repository::DuckDbRepository;
use crate::core::selector::{Internal, Operations, OrderDirection};
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
    let conn: duckdb::Connection = repo.get_connection()?;

    let mut query = Location::select();
    // let mut params: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(search_term) = &filters.search_term {
        query.filter(
            Internal::Field("name".into()),
            Operations::Like,
            Internal::Value(format!("%{}%", search_term).into()),
        );
        // query.push_str(" AND l.name LIKE ?");
    }

    // Add sorting
    if let Some(sort_by) = &filters.sort_by {
        if let Some(sort_order) = &filters.sort_order {
            // query.push_str(&format!(" ORDER BY l.{} {}", sort_by, sort_order));
            query.order(sort_by.to_owned().into(), sort_order.to_owned().into());
        } else {
            query.order(sort_by.to_owned().into(), OrderDirection::DESC);
        }
    } else {
        query.order("id".into(), OrderDirection::DESC);
    }

    // Add pagination
    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.paginate(limit, Some(offset));
    } else if let Some(limit) = filters.limit {
        query.paginate(limit, None);
    }
    let mut locations: Vec<Location> = Vec::new();
    let (stmt, translate) = query.all(&conn)?;

    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let location = Location::from_row(row, &translate);
        if let Ok(location) = location {
            if !(locations.iter().filter(|&e| e.id == location.id).count() > 0) {
                locations.push(location);
            }
        }
    }
    Ok(locations)
}

pub fn get_location_by_id(repo: &DuckDbRepository<Location>, id: &str) -> Result<Option<Location>> {
    let conn = repo.get_connection()?;

    let mut query = Location::select();
    query.filter(
        Internal::Field("id".into()),
        Operations::EqualTo,
        Internal::Value(id.to_string().into()),
    );

    let (stmt, translate) = query.all(&conn)?;
    let mut rows = stmt.raw_query();
    if let Some(row) = rows.next()? {
        let location: Location = Location::from_row(row, &translate)?;
        return Ok(Some(location));
    }
    Ok(None)
}

pub fn create_location(
    repo: &DuckDbRepository<Location>,
    location_dto: &LocationDto,
) -> Result<Location> {
    let conn = repo.get_connection()?;

    // Create the location with fields from DTO
    let mut location = Location::new(location_dto.name.clone());
    location.save(&conn)?;
    Ok(location)
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

    // Update fields
    location.name = location_dto.name.clone();

    location.save(&conn)?;
    Ok(location)
}
