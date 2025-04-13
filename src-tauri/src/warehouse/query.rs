use crate::core::error::Result;
use crate::core::repository::Repository;
use serde::Deserialize;
use std::error::Error;
use uuid::Uuid;
use chrono::Utc;

use super::model::Warehouse;
use super::inputs::WarehouseDto;

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
    filters: &FilterOptions
) -> Result<Vec<Warehouse>, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    let mut query = String::from("
        SELECT w.data AS warehouse_data
        FROM warehouse w
        WHERE 1=1
    ");

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    
    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND (JSON_EXTRACT(w.data, '$.name') LIKE ? OR JSON_EXTRACT(w.data, '$.shorthand') LIKE ?)");
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern.clone()));
        params.push(Box::new(search_pattern));
    }
    
    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        query.push_str(&format!(" ORDER BY JSON_EXTRACT(w.data, '$.{}') {}", sort_by, sort_order));
    } else {
        query.push_str(" ORDER BY JSON_EXTRACT(w.data, '$.updated_at') DESC");
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
    let mut rows = stmt.query(params.into_iter().map(|p| p.as_ref()).collect::<Vec<_>>().as_slice())?;

    let mut warehouses = Vec::new();
    while let Some(row) = rows.next()? {
        let warehouse_json: String = row.get(0)?;
        let warehouse: Warehouse = serde_json::from_str(&warehouse_json)?;
        warehouses.push(warehouse);
    }

    Ok(warehouses)
}

trait ToSql {
    fn as_ref(&self) -> &dyn rusqlite::ToSql;
}

impl<T: rusqlite::ToSql + ?Sized> ToSql for T {
    fn as_ref(&self) -> &dyn rusqlite::ToSql {
        self
    }
}

pub fn get_warehouse_by_id(
    repo: &impl Repository<Warehouse>,
    id: &str
) -> Result<Option<Warehouse>, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    let query = "SELECT data FROM warehouse WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let warehouse_json: String = row.get(0)?;
        let warehouse: Warehouse = serde_json::from_str(&warehouse_json)?;
        return Ok(Some(warehouse));
    }

    Ok(None)
}

pub fn create_warehouse(
    repo: &impl Repository<Warehouse>,
    warehouse_dto: &WarehouseDto
) -> Result<Warehouse, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Generate new UUID
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    // Create the warehouse with fields from DTO
    let warehouse = Warehouse {
        id,
        name: warehouse_dto.name.clone(),
        shorthand: warehouse_dto.shorthand.clone(),
        color: warehouse_dto.color.clone(), // This assumes WarehouseColorDto and WarehouseColor have same structure
        created_at: Some(now),
        updated_at: Some(now),
    };
    
    // Insert into the database
    let warehouse_json = serde_json::to_string(&warehouse)?;
    let insert_sql = "INSERT INTO warehouse (data) VALUES (?)";
    let mut stmt = conn.prepare(insert_sql)?;
    stmt.execute(&[&warehouse_json])?;
    
    Ok(warehouse)
}

pub fn update_warehouse(
    repo: &impl Repository<Warehouse>,
    id: &str,
    warehouse_dto: &WarehouseDto
) -> Result<Warehouse, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Check if warehouse exists
    let warehouse_check = get_warehouse_by_id(repo, id)?;
    if warehouse_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Warehouse with id {} not found", id)
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
    let warehouse_json = serde_json::to_string(&warehouse)?;
    let update_sql = "UPDATE warehouse SET data = ? WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(update_sql)?;
    stmt.execute(&[&warehouse_json, id])?;
    
    Ok(warehouse)
} 