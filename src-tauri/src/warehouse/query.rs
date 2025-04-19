use crate::core::error::{custom_error, Result};
use crate::core::repository::DuckDbRepository;
use crate::core::selector::{Internal, Operations, OrderDirection};
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
    repo: &DuckDbRepository<Warehouse>,
    filters: &FilterOptions,
) -> Result<Vec<Warehouse>> {
    let conn = repo.get_connection()?;

    let mut query = Warehouse::select();

    if let Some(search_term) = &filters.search_term {
        query
            .filter(
                Internal::Field("name".into()),
                Operations::Like,
                Internal::Value(format!("%{}%", search_term).into()),
            )
            .or(|cond| {
                cond.filter(
                    Internal::Field("shorthand".into()),
                    Operations::Like,
                    Internal::Value(format!("%{}%", search_term).into()),
                );
            });
    }

    // Add sorting
    if let Some(sort_by) = &filters.sort_by {
        if let Some(sort_order) = &filters.sort_order {
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

    let mut warehouses: Vec<Warehouse> = Vec::new();
    let (stmt, translate) = query.all(&conn)?;
    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let warehouse = Warehouse::from_row(row, &translate);
        if let Ok(warehouse) = warehouse {
            // Don't add duplicate warehouses
            if !(warehouses.iter().filter(|&e| e.id == warehouse.id).count() > 0) {
                warehouses.push(warehouse);
            }
        }
    }
    Ok(warehouses)
}

pub fn get_warehouse_by_id(
    repo: &DuckDbRepository<Warehouse>,
    id: &str,
) -> Result<Option<Warehouse>> {
    let conn = repo.get_connection()?;

    let mut query = Warehouse::select();

    query.filter(
        Internal::Field("id".into()),
        Operations::EqualTo,
        Internal::Value(id.to_string().into()),
    );

    let result = query.all(&conn);
    if let Ok((stmt, translate)) = result {
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let warehouse: Warehouse = Warehouse::from_row(row, &translate)?;
            return Ok(Some(warehouse));
        }
    }
    Ok(None)
}

pub fn create_warehouse(
    repo: &DuckDbRepository<Warehouse>,
    warehouse_dto: &WarehouseDto,
) -> Result<Warehouse> {
    let conn = repo.get_connection()?;

    // Create the warehouse with fields from DTO
    let mut warehouse = Warehouse::new(
        warehouse_dto.name.clone(),
        warehouse_dto.shorthand.clone(),
        warehouse_dto.color.clone(),
    );
    warehouse.save(&conn)?;
    Ok(warehouse)
}

pub fn update_warehouse(
    repo: &DuckDbRepository<Warehouse>,
    id: &str,
    warehouse_dto: &WarehouseDto,
) -> Result<Warehouse> {
    let conn = repo.get_connection()?;

    // Check if warehouse exists
    let warehouse_check = get_warehouse_by_id(repo, id)?;
    if warehouse_check.is_none() {
        return Err(custom_error(format!("Warehouse with id {} not found", id)));
    }

    let mut warehouse = warehouse_check.unwrap();

    // Update fields
    warehouse.name = warehouse_dto.name.clone();
    warehouse.shorthand = warehouse_dto.shorthand.clone();
    warehouse.color = warehouse_dto.color.clone();
    warehouse.save(&conn)?;
    Ok(warehouse)
}
