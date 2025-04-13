use crate::core::error::{custom_error, Result};
use crate::core::repository::Repository;
use chrono::Utc;
use duckdb::ToSql;
use serde::Deserialize;

use super::inputs::WarehouseDto;
use super::model::Warehouse;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_term: Option<String>,
}

pub fn get_warehouse_with_filter(
    repo: &impl Repository<Warehouse>,
    filters: &FilterOptions,
) -> Result<Vec<Warehouse>> {
    let conn = repo.get_connection()?;

    let mut query = String::from(format!(
        "
            SELECT
                {}
            FROM warehouse w
            WHERE 1=1
        ",
        Warehouse::get_select_for("w".to_string(), "".to_string())
    ));

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();

    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND w.name LIKE ? OR w.shorthand LIKE ?");
        // Use the same search term for both name and shorthand
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern.clone()));
        params.push(Box::new(search_pattern));
    }

    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        query.push_str(&format!(" ORDER BY w.{} {}", sort_by, sort_order));
    } else {
        query.push_str(" ORDER BY w.updated_at DESC");
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

    let mut warehouses = Vec::new();
    while let Some(row) = rows.next()? {
        let warehouse = Warehouse::from_row(row, &stmt);
        if !(warehouses.iter().filter(|&e| e.id == warehouse.id).count() > 0) {
            warehouses.push(warehouse);
        }
    }

    Ok(warehouses)
}

pub fn get_warehouse_by_id(
    repo: &impl Repository<Warehouse>,
    id: &str,
) -> Result<Option<Warehouse>> {
    let conn = repo.get_connection()?;

    let query = format!(
        "SELECT {} FROM warehouse w WHERE w.id = ?",
        Warehouse::get_select_for("w".to_string(), "".to_string())
    );
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let warehouse: Warehouse = Warehouse::from_row(row, &stmt);
        return Ok(Some(warehouse));
    }

    Ok(None)
}

pub fn create_warehouse(
    repo: &impl Repository<Warehouse>,
    warehouse_dto: &WarehouseDto,
) -> Result<Warehouse> {
    let conn = repo.get_connection()?;

    // Generate new UUID
    let now = Utc::now();

    // Create the warehouse with fields from DTO
    let warehouse = Warehouse {
        id: -1,
        name: warehouse_dto.name.clone(),
        shorthand: warehouse_dto.shorthand.clone(),
        color: warehouse_dto.color.clone(), // This assumes WarehouseColorDto and WarehouseColor have same structure
        created_at: Some(now),
        updated_at: Some(now),
    };

    // Insert into the database
    let (insert_sql, data) = warehouse.get_insert_query();
    let mut stmt = conn.prepare(insert_sql)?;
    stmt.execute(&[&data])?;

    if insert_sql.len() > 0 {
        let mut stmt = conn.prepare(&insert_sql)?;
        stmt.insert(&data)?;

        return match stmt.raw_query().next()? {
            Some(row) => {
                let id = row.get(stmt.column_index("id")?)?;
                warehouse.id = id;
                Ok(warehouse)
            }
            None => Err(custom_error("failed to insert at 'warehouse'!")),
        };
        Err(custom_error(
            "Failed to get insert query for Warehouse model!",
        ))
    }
}

pub fn update_warehouse(
    repo: &impl Repository<Warehouse>,
    id: &str,
    warehouse_dto: &WarehouseDto,
) -> Result<Warehouse> {
    let conn = repo.get_connection()?;

    // Check if warehouse exists
    let warehouse_check = get_warehouse_by_id(repo, id)?;
    if warehouse_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Warehouse with id {} not found", id),
        )));
    }

    let mut warehouse = warehouse_check.unwrap();
    let now = Utc::now();

    // Update fields
    warehouse.name = warehouse_dto.name.clone();
    warehouse.shorthand = warehouse_dto.shorthand.clone();
    warehouse.color = warehouse_dto.color.clone();
    warehouse.updated_at = Some(now);

    // Update in the database
    let (update_sql, data) = warehouse.get_update_query();
    if update_sql.len() > 0 {
        let mut stmt = conn.prepare(&update_sql)?;
        stmt.update(&data)?;
        Ok(warehouse)
    }
    Err(custom_error(
        "Failed to get update query for Location model!",
    ))
}
