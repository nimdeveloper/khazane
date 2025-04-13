use crate::core::error::Result;
use crate::core::repository::Repository;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use chrono::Utc;

use super::model::{Product, ProductCategory, MeasurementUnit, ProductWarehouse};
use super::inputs::{ProductDto, ProductCategoryDto, MeasurementUnitDto};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub status: Option<String>,
    pub category_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_term: Option<String>,
}

pub fn get_product_with_filter(
    repo: &impl Repository<Product>,
    filters: &FilterOptions
) -> Result<Vec<Product>, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    let mut query = String::from("
        SELECT p.id, p.title, p.code, p.base_price, p.inventory, p.initial_inventory, 
               p.status, p.image, p.created_at, p.updated_at,
               c.id AS category_id, c.label AS category_label, c.created_at AS category_created_at, c.updated_at AS category_updated_at,
               u.id AS unit_id, u.title AS unit_title
        FROM product p
        LEFT JOIN category c ON p.category_id = c.id
        LEFT JOIN measure_unit u ON p.unit_id = u.id
        WHERE 1=1
    ");

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    
    if let Some(status) = &filters.status {
        query.push_str(" AND p.status = ?");
        params.push(Box::new(status.clone()));
    }
    
    if let Some(category_id) = &filters.category_id {
        query.push_str(" AND p.category_id = ?");
        params.push(Box::new(category_id.clone()));
    }
    
    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND (p.title LIKE ? OR p.code LIKE ?)");
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern.clone()));
        params.push(Box::new(search_pattern));
    }
    
    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        query.push_str(&format!(" ORDER BY p.{} {}", sort_by, sort_order));
    } else {
        query.push_str(" ORDER BY p.updated_at DESC");
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

    let mut products = Vec::new();
    while let Some(row) = rows.next()? {
        let mut product = Product {
            id: row.get(0)?,
            title: row.get(1)?,
            code: row.get(2)?,
            base_price: row.get(3)?,
            inventory: row.get(4)?,
            initial_inventory: row.get(5)?,
            status: row.get(6)?,
            image: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
            unit: None,
            category: None,
            ware_houses: Vec::new(),
        };
        
        // Parse category if available
        if let Ok(category_id) = row.get::<_, Option<String>>(10) {
            if let Some(category_id) = category_id {
                product.category = Some(ProductCategory {
                    id: category_id,
                    label: row.get(11)?,
                    created_at: row.get(12)?,
                    updated_at: row.get(13)?,
                });
            }
        }
        
        // Parse unit if available
        if let Ok(unit_id) = row.get::<_, Option<String>>(14) {
            if let Some(unit_id) = unit_id {
                product.unit = Some(MeasurementUnit {
                    id: unit_id,
                    title: row.get(15)?,
                });
            }
        }
        
        // Load warehouses for this product in a separate query
        let warehouse_query = "
            SELECT pw.quantity, w.id, w.name, w.shorthand, w.color_key, w.color_code
            FROM product_warehouse pw
            JOIN warehouse w ON pw.warehouse_id = w.id
            WHERE pw.product_id = ?
        ";
        
        let mut warehouse_stmt = conn.prepare(warehouse_query)?;
        let mut warehouse_rows = warehouse_stmt.query(&[&product.id])?;
        
        let mut warehouses = Vec::new();
        while let Some(warehouse_row) = warehouse_rows.next()? {
            let quantity: i64 = warehouse_row.get(0)?;
            let warehouse = Warehouse {
                id: warehouse_row.get(1)?,
                name: warehouse_row.get(2)?,
                shorthand: warehouse_row.get(3)?,
                color_key: warehouse_row.get(4)?,
                color_code: warehouse_row.get(5)?,
            };
            
            warehouses.push(ProductWarehouse {
                quantity,
                warehouse: Some(warehouse),
            });
        }
        
        product.ware_houses = warehouses;
        products.push(product);
    }

    Ok(products)
}

pub fn get_product_by_id(
    repo: &impl Repository<Product>,
    id: &str
) -> Result<Option<Product>, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    let query = "
        SELECT p.id, p.title, p.code, p.base_price, p.inventory, p.initial_inventory, 
               p.status, p.image, p.created_at, p.updated_at,
               c.id AS category_id, c.label AS category_label, c.created_at AS category_created_at, c.updated_at AS category_updated_at,
               u.id AS unit_id, u.title AS unit_title
        FROM product p
        LEFT JOIN category c ON p.category_id = c.id
        LEFT JOIN measure_unit u ON p.unit_id = u.id
        WHERE p.id = ?
    ";
    
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;
    
    if let Some(row) = rows.next()? {
        let mut product = Product {
            id: row.get(0)?,
            title: row.get(1)?,
            code: row.get(2)?,
            base_price: row.get(3)?,
            inventory: row.get(4)?,
            initial_inventory: row.get(5)?,
            status: row.get(6)?,
            image: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
            unit: None,
            category: None,
            ware_houses: Vec::new(),
        };
        
        // Parse category if available
        if let Ok(category_id) = row.get::<_, Option<String>>(10) {
            if let Some(category_id) = category_id {
                product.category = Some(ProductCategory {
                    id: category_id,
                    label: row.get(11)?,
                    created_at: row.get(12)?,
                    updated_at: row.get(13)?,
                });
            }
        }
        
        // Parse unit if available
        if let Ok(unit_id) = row.get::<_, Option<String>>(14) {
            if let Some(unit_id) = unit_id {
                product.unit = Some(MeasurementUnit {
                    id: unit_id,
                    title: row.get(15)?,
                });
            }
        }
        
        // Load warehouses for this product
        let warehouse_query = "
            SELECT pw.quantity, w.id, w.name, w.shorthand, w.color_key, w.color_code
            FROM product_warehouse pw
            JOIN warehouse w ON pw.warehouse_id = w.id
            WHERE pw.product_id = ?
        ";
        
        let mut warehouse_stmt = conn.prepare(warehouse_query)?;
        let mut warehouse_rows = warehouse_stmt.query(&[&product.id])?;
        
        let mut warehouses = Vec::new();
        while let Some(warehouse_row) = warehouse_rows.next()? {
            let quantity: i64 = warehouse_row.get(0)?;
            let warehouse = Warehouse {
                id: warehouse_row.get(1)?,
                name: warehouse_row.get(2)?,
                shorthand: warehouse_row.get(3)?,
                color_key: warehouse_row.get(4)?,
                color_code: warehouse_row.get(5)?,
            };
            
            warehouses.push(ProductWarehouse {
                quantity,
                warehouse: Some(warehouse),
            });
        }
        
        product.ware_houses = warehouses;
        Ok(Some(product))
    } else {
        Ok(None)
    }
}

pub fn create_product(
    repo: &impl Repository<Product>,
    product_dto: &ProductDto
) -> Result<Product, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Generate new UUID
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    // Create the product with basic fields
    let mut product = Product {
        id,
        title: product_dto.title.clone(),
        code: product_dto.code.clone(),
        base_price: product_dto.base_price,
        inventory: product_dto.initial_inventory,
        initial_inventory: product_dto.initial_inventory,
        status: product_dto.status.clone(),
        image: product_dto.image.clone(),
        unit: None,
        category: None,
        ware_houses: Vec::new(),
        created_at: Some(now),
        updated_at: Some(now),
    };
    
    // Handle category relation if provided
    if let Some(category_id) = &product_dto.category_id {
        if !category_id.is_empty() {
            // Check if category exists
            let category_query = "SELECT data FROM category WHERE JSON_EXTRACT(data, '$.id') = ?";
            let mut category_stmt = conn.prepare(category_query)?;
            if let Some(category_row) = category_stmt.query(&[category_id])?.next()? {
                let category_json: String = category_row.get(0)?;
                let category: ProductCategory = serde_json::from_str(&category_json)?;
                product.category = Some(category);
            }
        }
    }
    
    // Handle unit relation if provided
    if let Some(unit_id) = &product_dto.unit_id {
        if !unit_id.is_empty() {
            // Check if unit exists
            let unit_query = "SELECT data FROM measure_unit WHERE JSON_EXTRACT(data, '$.id') = ?";
            let mut unit_stmt = conn.prepare(unit_query)?;
            if let Some(unit_row) = unit_stmt.query(&[unit_id])?.next()? {
                let unit_json: String = unit_row.get(0)?;
                let unit: MeasurementUnit = serde_json::from_str(&unit_json)?;
                product.unit = Some(unit);
            }
        }
    }
    
    // Insert the product
    let product_json = serde_json::to_string(&product)?;
    let insert_product_sql = "INSERT INTO product (data) VALUES (?)";
    let mut stmt = conn.prepare(insert_product_sql)?;
    stmt.execute(&[&product_json])?;
    
    // Handle warehouse relations if any
    if let Some(warehouses) = &product_dto.warehouses {
        for warehouse_dto in warehouses {
            // Create warehouse relation
            let warehouse_id = warehouse_dto.warehouse_id.clone();
            if !warehouse_id.is_empty() {
                let product_warehouse_id = Uuid::new_v4().to_string();
                let product_warehouse = ProductWarehouse {
                    quantity: warehouse_dto.quantity,
                    warehouse: None, // We'll load this when querying
                };
                
                // Store relation with IDs
                let product_warehouse_json = serde_json::json!({
                    "id": product_warehouse_id,
                    "quantity": product_warehouse.quantity,
                    "product_id": product.id,
                    "warehouse_id": warehouse_id,
                    "created_at": now.to_rfc3339(),
                    "updated_at": now.to_rfc3339()
                });
                
                let insert_warehouse_sql = "INSERT INTO product_warehouse (data) VALUES (?)";
                let mut wh_stmt = conn.prepare(insert_warehouse_sql)?;
                wh_stmt.execute(&[&product_warehouse_json.to_string()])?;
            }
        }
    }
    
    // Return the complete product with relations
    get_product_by_id(repo, &product.id)?.ok_or(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Product not found after creation"
    )))
}

pub fn update_product(
    repo: &impl Repository<Product>,
    id: &str,
    product_dto: &ProductDto
) -> Result<Product, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Check if product exists
    let product_check = get_product_by_id(repo, id)?;
    if product_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Product with id {} not found", id)
        )));
    }
    
    let mut product = product_check.unwrap();
    let now = Utc::now();
    
    // Update basic fields
    product.title = product_dto.title.clone();
    product.code = product_dto.code.clone();
    product.base_price = product_dto.base_price;
    product.initial_inventory = product_dto.initial_inventory;
    product.status = product_dto.status.clone();
    product.image = product_dto.image.clone();
    product.updated_at = Some(now);
    
    // Handle category relation if provided and ID changed
    if let Some(category_id) = &product_dto.category_id {
        let current_category_id = product.category.as_ref().map(|c| c.id.clone());
        
        if current_category_id != Some(category_id.clone()) && !category_id.is_empty() {
            // Category ID changed, look up new category
            let category_query = "SELECT data FROM category WHERE JSON_EXTRACT(data, '$.id') = ?";
            let mut category_stmt = conn.prepare(category_query)?;
            if let Some(category_row) = category_stmt.query(&[category_id])?.next()? {
                let category_json: String = category_row.get(0)?;
                let category: ProductCategory = serde_json::from_str(&category_json)?;
                product.category = Some(category);
            } else {
                product.category = None;
            }
        } else if category_id.is_empty() {
            product.category = None;
        }
    }
    
    // Handle unit relation if provided and ID changed
    if let Some(unit_id) = &product_dto.unit_id {
        let current_unit_id = product.unit.as_ref().map(|u| u.id.clone());
        
        if current_unit_id != Some(unit_id.clone()) && !unit_id.is_empty() {
            // Unit ID changed, look up new unit
            let unit_query = "SELECT data FROM measure_unit WHERE JSON_EXTRACT(data, '$.id') = ?";
            let mut unit_stmt = conn.prepare(unit_query)?;
            if let Some(unit_row) = unit_stmt.query(&[unit_id])?.next()? {
                let unit_json: String = unit_row.get(0)?;
                let unit: MeasurementUnit = serde_json::from_str(&unit_json)?;
                product.unit = Some(unit);
            } else {
                product.unit = None;
            }
        } else if unit_id.is_empty() {
            product.unit = None;
        }
    }
    
    // Update the product
    let product_json = serde_json::to_string(&product)?;
    let update_product_sql = "UPDATE product SET data = ? WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(update_product_sql)?;
    stmt.execute(&[&product_json, id])?;
    
    // Handle warehouse relations if any
    if let Some(warehouses) = &product_dto.warehouses {
        // First, delete all existing warehouse relations
        let delete_warehouses_sql = "DELETE FROM product_warehouse WHERE JSON_EXTRACT(data, '$.product_id') = ?";
        let mut delete_stmt = conn.prepare(delete_warehouses_sql)?;
        delete_stmt.execute(&[id])?;
        
        // Create new warehouse relations
        for warehouse_dto in warehouses {
            let warehouse_id = warehouse_dto.warehouse_id.clone();
            if !warehouse_id.is_empty() {
                let product_warehouse_id = Uuid::new_v4().to_string();
                
                // Store relation with IDs
                let product_warehouse_json = serde_json::json!({
                    "id": product_warehouse_id,
                    "quantity": warehouse_dto.quantity,
                    "product_id": id,
                    "warehouse_id": warehouse_id,
                    "created_at": now.to_rfc3339(),
                    "updated_at": now.to_rfc3339()
                });
                
                let insert_warehouse_sql = "INSERT INTO product_warehouse (data) VALUES (?)";
                let mut wh_stmt = conn.prepare(insert_warehouse_sql)?;
                wh_stmt.execute(&[&product_warehouse_json.to_string()])?;
            }
        }
    }
    
    // Return the updated product with relations
    get_product_by_id(repo, id)?.ok_or(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Product not found after update"
    )))
}

// Category functions
pub fn get_category_by_id(
    repo: &impl Repository<ProductCategory>,
    id: &str
) -> Result<Option<ProductCategory>, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    let query = "SELECT data FROM category WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let category_json: String = row.get(0)?;
        let category: ProductCategory = serde_json::from_str(&category_json)?;
        return Ok(Some(category));
    }

    Ok(None)
}

pub fn create_category(
    repo: &impl Repository<ProductCategory>,
    category_dto: &ProductCategoryDto
) -> Result<ProductCategory, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Generate new UUID
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    // Create the category
    let category = ProductCategory {
        id,
        label: category_dto.label.clone(),
        created_at: Some(now),
        updated_at: Some(now),
    };
    
    // Insert the category
    let category_json = serde_json::to_string(&category)?;
    let insert_sql = "INSERT INTO category (data) VALUES (?)";
    let mut stmt = conn.prepare(insert_sql)?;
    stmt.execute(&[&category_json])?;
    
    Ok(category)
}

pub fn update_category(
    repo: &impl Repository<ProductCategory>,
    id: &str,
    category_dto: &ProductCategoryDto
) -> Result<ProductCategory, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Check if category exists
    let category_check = get_category_by_id(repo, id)?;
    if category_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Category with id {} not found", id)
        )));
    }
    
    let mut category = category_check.unwrap();
    let now = Utc::now();
    
    // Update fields
    category.label = category_dto.label.clone();
    category.updated_at = Some(now);
    
    // Update the category
    let category_json = serde_json::to_string(&category)?;
    let update_sql = "UPDATE category SET data = ? WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(update_sql)?;
    stmt.execute(&[&category_json, id])?;
    
    Ok(category)
}

// Measurement Unit functions
pub fn get_measurement_unit_by_id(
    repo: &impl Repository<MeasurementUnit>,
    id: &str
) -> Result<Option<MeasurementUnit>, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    let query = "SELECT data FROM measure_unit WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let unit_json: String = row.get(0)?;
        let unit: MeasurementUnit = serde_json::from_str(&unit_json)?;
        return Ok(Some(unit));
    }

    Ok(None)
}

pub fn create_measurement_unit(
    repo: &impl Repository<MeasurementUnit>,
    unit_dto: &MeasurementUnitDto
) -> Result<MeasurementUnit, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Generate new UUID
    let id = Uuid::new_v4().to_string();
    
    // Create the unit
    let unit = MeasurementUnit {
        id,
        title: unit_dto.title.clone(),
    };
    
    // Insert the unit
    let unit_json = serde_json::to_string(&unit)?;
    let insert_sql = "INSERT INTO measure_unit (data) VALUES (?)";
    let mut stmt = conn.prepare(insert_sql)?;
    stmt.execute(&[&unit_json])?;
    
    Ok(unit)
}

pub fn update_measurement_unit(
    repo: &impl Repository<MeasurementUnit>,
    id: &str,
    unit_dto: &MeasurementUnitDto
) -> Result<MeasurementUnit, Box<dyn Error>> {
    let conn = repo.get_connection()?;
    
    // Check if unit exists
    let unit_check = get_measurement_unit_by_id(repo, id)?;
    if unit_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Measurement unit with id {} not found", id)
        )));
    }
    
    let mut unit = unit_check.unwrap();
    
    // Update fields
    unit.title = unit_dto.title.clone();
    
    // Update the unit
    let unit_json = serde_json::to_string(&unit)?;
    let update_sql = "UPDATE measure_unit SET data = ? WHERE JSON_EXTRACT(data, '$.id') = ?";
    let mut stmt = conn.prepare(update_sql)?;
    stmt.execute(&[&unit_json, id])?;
    
    Ok(unit)
}

trait ToSql {
    fn as_ref(&self) -> &dyn rusqlite::ToSql;
}

impl<T: rusqlite::ToSql + ?Sized> ToSql for T {
    fn as_ref(&self) -> &dyn rusqlite::ToSql {
        self
    }
} 