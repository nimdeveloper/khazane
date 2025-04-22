use crate::core::error::{custom_error, Result};
use crate::core::repository::DuckDbRepository;
use crate::core::selector::{Internal, Operations, OrderDirection};
use crate::warehouse::model::Warehouse;
use serde::Deserialize;

use super::inputs::{MeasurementUnitDto, ProductCategoryDto, ProductDto};
use super::model::{MeasurementUnit, Product, ProductCategory, ProductWarehouse};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub status: Option<String>,
    pub category_id: Option<i64>,
    pub warehouse_id: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_term: Option<String>,
}

pub fn get_product_with_filter(
    repo: &DuckDbRepository<Product>,
    filters: &FilterOptions,
) -> Result<Vec<Product>> {
    let conn = repo.get_connection()?;

    let mut query = Product::select();

    if let Some(status) = &filters.status {
        query.filter(
            Internal::Field("status".into()),
            Operations::EqualTo,
            Internal::Value(status.clone().into()),
        );
        // query.push_str(" AND p.status = ?");
    }

    if let Some(category_id) = &filters.category_id {
        query.filter(
            Internal::Field("category_id".into()),
            Operations::EqualTo,
            Internal::Value(category_id.clone().into()),
        );
    }

    if let Some(warehouse_id) = &filters.warehouse_id {
        query.filter(
            Internal::Field("warehouse_id".into()),
            Operations::EqualTo,
            Internal::Value(warehouse_id.clone().into()),
        );
    }

    if let Some(search_term) = &filters.search_term {
        query
            .filter(
                Internal::Field("title".into()),
                Operations::Like,
                Internal::Value(format!("%{}%", search_term).into()),
            )
            .or(|cond| {
                cond.filter(
                    Internal::Field("code".into()),
                    Operations::Like,
                    Internal::Value(format!("%{}%", search_term).into()),
                );
            });
        // query.push_str(" AND (p.title LIKE ? OR p.code LIKE ?)");
        // let search_pattern = format!("%{}%", search_term);
        // params.push(Box::new(search_pattern.clone()));
        // params.push(Box::new(search_pattern));
    }

    // Add sorting
    if let Some(sort_by) = &filters.sort_by {
        if let Some(sort_order) = &filters.sort_order {
            query.order(sort_by.to_owned().into(), sort_order.to_owned().into());
            // query.push_str(&format!(" ORDER BY p.{} {}", sort_by, sort_order));
        } else {
            query.order(sort_by.to_owned().into(), OrderDirection::DESC);
        }
    } else {
        query.order("created_at".into(), OrderDirection::DESC);
    }

    // Add pagination
    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.paginate(limit, Some(offset));
        // query.push_str(" LIMIT ? OFFSET ?");
    } else if let Some(limit) = filters.limit {
        query.paginate(limit, None);
    }

    let mut products: Vec<Product> = Vec::new();
    let (stmt, translate) = query.all(&conn)?;
    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let mut product = Product::from_row(row, &translate)?;
        if !(products.iter().filter(|&e| e.id == product.id).count() > 0) {
            product.related_from_row(row, &translate)?;
            products.push(product);
        } else {
            let current_product = products.iter_mut().find(|e| e.id == product.id).unwrap();
            current_product.related_from_row(row, &translate)?;
        }
    }
    Product::bulk_warehouses_load(&conn, &mut products)?;
    Ok(products)
}

pub fn get_product_by_id(repo: &DuckDbRepository<Product>, id: i64) -> Result<Option<Product>> {
    let conn = repo.get_connection()?;

    let mut query = Product::select();
    query.filter(
        Internal::Field("id".into()),
        Operations::EqualTo,
        Internal::Value(id.into()),
    );
    let (stmt, translate) = query.all(&conn)?;
    let mut rows = stmt.raw_query();
    let mut product: Option<Product> = None;
    if let Some(row) = rows.next()? {
        if product.is_none() {
            product = Some(Product::from_row(row, &translate)?);
        }
        let product = product.as_mut().unwrap();
        product.related_from_row(row, &translate)?;
        product.load_warehouses(&conn)?;
    }
    Ok(product)
}

pub fn create_product(
    repo: &DuckDbRepository<Product>,
    product_dto: &ProductDto,
) -> Result<Product> {
    let conn = repo.get_connection()?;

    // Create the product with basic fields
    let mut product = Product::new(
        product_dto.title.clone(),
        product_dto.code.clone(),
        product_dto.base_price,
        product_dto.inventory,
        product_dto.initial_inventory,
        product_dto.status.clone(),
        product_dto.image.clone(),
    );

    // Handle category relation if provided
    if let Some(category) = &product_dto.category {
        let category_id = category.id.unwrap();
        // Check if category exists
        let mut category_query = ProductCategory::select();
        category_query.filter(
            Internal::Field("id".into()),
            Operations::EqualTo,
            Internal::Value(category_id.clone().into()),
        );

        let (stmt, translate) = category_query.all(&conn)?;
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let category: ProductCategory = ProductCategory::from_row(row, &translate)?;
            product.category = Some(category);
        }
    }

    // Handle unit relation if provided
    if let Some(unit) = &product_dto.unit {
        let unit_id = unit.id.unwrap();
        // Check if unit exists
        let mut unit_query = MeasurementUnit::select();
        unit_query.filter(
            Internal::Field("id".into()),
            Operations::EqualTo,
            Internal::Value(unit_id.clone().into()),
        );

        let (stmt, translate) = unit_query.all(&conn)?;
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let unit: MeasurementUnit = MeasurementUnit::from_row(row, &translate)?;
            product.unit = Some(unit);
        }
    }

    product.save(&conn)?;
    // Insert the product
    // let product_json = serde_json::to_string(&product)?;
    // let insert_product_sql = "INSERT INTO product (data) VALUES (?)";
    // let mut stmt = conn.prepare(insert_product_sql)?;
    // stmt.execute(&[&product_json])?;

    // Handle warehouse relations if any
    let mut fetched_warehouses: Vec<Warehouse> = Vec::new();

    let mut warehouse_query = Warehouse::select();

    //
    //
    // !
    // ! WARN: Fix after `List` is implemented by Duckdb!
    // !
    //
    if product_dto.warehouses.len() > 0 {
        let ids = product_dto
            .warehouses
            .iter()
            .filter(|e| e.warehouse.is_some())
            .map(|e| e.warehouse.clone().unwrap().id.unwrap())
            .collect::<Vec<i64>>();

        let unstable_list: String = ids
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let unstable_condition: String = format!("({})", unstable_list);
        warehouse_query.filter(
            Internal::Field("id".into()),
            Operations::In,
            Internal::Field(unstable_condition.into()),
        );
    }
    // !
    // !END
    // !
    // warehouse_query.filter(
    //     Internal::Field("id".into()),
    //     Operations::In,
    //     Internal::Value(
    //         duckdb::types::Value::List(
    //             product_dto
    //                 .warehouses
    //                 .iter()
    //                 .map(|w| duckdb::types::Value::BigInt(w.id.clone()))
    //                 .collect::<Vec<duckdb::types::Value>>(),
    //         )
    //         .into(),
    //     ),
    // );
    if product_dto.warehouses.len() > 0 {
        let (stmt, translate) = warehouse_query.all(&conn)?;
        let mut rows = stmt.raw_query();
        while let Some(row) = rows.next()? {
            let warehouse: Warehouse = Warehouse::from_row(row, &translate)?;
            fetched_warehouses.push(warehouse);
        }
    }
    let mut to_add_warehouses: Vec<ProductWarehouse> = Vec::new();
    // Create warehouse relations
    for warehouse in product_dto.warehouses.iter() {
        let mut instance_warehouse = None;
        if warehouse.warehouse.is_some() {
            let warehouse_db_instance = fetched_warehouses
                .iter()
                .find(|w| w.id == warehouse.warehouse.clone().unwrap().id.unwrap());
            if warehouse_db_instance.is_some() {
                instance_warehouse = Some(warehouse_db_instance.unwrap().clone())
            }
        }

        let new_warehouse =
            ProductWarehouse::new(warehouse.quantity, product.id, instance_warehouse);
        to_add_warehouses.push(new_warehouse);
    }

    ProductWarehouse::bulk_insert(&to_add_warehouses, &conn)?;

    // Return the complete product with relations
    get_product_by_id(repo, product.id)?.ok_or(custom_error("Product not found!"))
}

pub fn update_product(
    repo: &DuckDbRepository<Product>,
    id: i64,
    product_dto: &ProductDto,
) -> Result<Product> {
    let conn = repo.get_connection()?;

    // Check if product exists
    let product_check = get_product_by_id(repo, id)?;
    if product_check.is_none() {
        return Err(custom_error(format!("Product with id {} not found", id)));
    }

    let mut product = product_check.unwrap();

    // Update basic fields
    product.title = product_dto.title.clone();
    product.code = product_dto.code.clone();
    product.base_price = product_dto.base_price;
    product.inventory = product_dto.inventory;
    product.initial_inventory = product_dto.initial_inventory;
    product.status = product_dto.status.clone();
    product.image = product_dto.image.clone();

    if let Some(category) = &product_dto.category {
        let category_id = category.id.unwrap();
        // Check if category exists
        let mut category_query = ProductCategory::select();
        category_query.filter(
            Internal::Field("id".into()),
            Operations::EqualTo,
            Internal::Value(category_id.clone().into()),
        );

        let (stmt, translate) = category_query.all(&conn)?;
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let category: ProductCategory = ProductCategory::from_row(row, &translate)?;
            product.category = Some(category);
        }
    } else {
        product.category = None
    }

    // Handle unit relation if provided
    if let Some(unit) = &product_dto.unit {
        let unit_id = unit.id.unwrap();
        // Check if unit exists
        let mut unit_query = MeasurementUnit::select();
        unit_query.filter(
            Internal::Field("id".into()),
            Operations::EqualTo,
            Internal::Value(unit_id.clone().into()),
        );

        let (stmt, translate) = unit_query.all(&conn)?;
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let unit: MeasurementUnit = MeasurementUnit::from_row(row, &translate)?;
            product.unit = Some(unit);
        }
    } else {
        product.unit = None
    }

    // Handle warehouse relations if any
    // if let Some(warehouses) = &product_dto.warehouses {
    //     // First, delete all existing warehouse relations
    //     let delete_warehouses_sql =
    //         "DELETE FROM product_warehouse WHERE JSON_EXTRACT(data, '$.product_id') = ?";
    //     let mut delete_stmt = conn.prepare(delete_warehouses_sql)?;
    //     delete_stmt.execute(&[id])?;

    //     // Create new warehouse relations
    //     for warehouse_dto in warehouses {
    //         let warehouse_id = warehouse_dto.warehouse_id.clone();
    //         if !warehouse_id.is_empty() {
    //             let product_warehouse_id = Uuid::new_v4().to_string();

    //             // Store relation with IDs
    //             let product_warehouse_json = serde_json::json!({
    //                 "id": product_warehouse_id,
    //                 "quantity": warehouse_dto.quantity,
    //                 "product_id": id,
    //                 "warehouse_id": warehouse_id,
    //                 "created_at": now.to_rfc3339(),
    //                 "updated_at": now.to_rfc3339()
    //             });

    //             let insert_warehouse_sql = "INSERT INTO product_warehouse (data) VALUES (?)";
    //             let mut wh_stmt = conn.prepare(insert_warehouse_sql)?;
    //             wh_stmt.execute(&[&product_warehouse_json.to_string()])?;
    //         }
    //     }
    // }

    // Return the updated product with relations
    product.save(&conn)?;
    get_product_by_id(repo, id)?.ok_or(custom_error("Product not found after update"))
}

// Categories

pub fn get_category_with_filter(
    repo: &DuckDbRepository<ProductCategory>,
    filters: &FilterOptions,
) -> Result<Vec<ProductCategory>> {
    let conn: duckdb::Connection = repo.get_connection()?;

    let mut query = ProductCategory::select();

    if let Some(search_term) = &filters.search_term {
        query.filter(
            Internal::Field("label".into()),
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
        query.order("created_at".into(), OrderDirection::DESC);
    }

    // Add pagination
    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.paginate(limit, Some(offset));
    } else if let Some(limit) = filters.limit {
        query.paginate(limit, None);
    }
    let mut categories: Vec<ProductCategory> = Vec::new();
    let (stmt, translate) = query.all(&conn)?;

    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let category = ProductCategory::from_row(row, &translate);
        if let Ok(category) = category {
            if !(categories.iter().filter(|&e| e.id == category.id).count() > 0) {
                categories.push(category);
            }
        }
    }
    Ok(categories)
}

pub fn get_category_by_id(
    repo: &DuckDbRepository<ProductCategory>,
    id: i64,
) -> Result<Option<ProductCategory>> {
    let conn = repo.get_connection()?;

    let mut query = ProductCategory::select();
    query.filter(
        Internal::Field("id".into()),
        Operations::EqualTo,
        Internal::Value(id.into()),
    );

    let result = query.all(&conn);
    if let Ok((stmt, translate)) = result {
        let mut rows = stmt.raw_query();
        if let Some(row) = rows.next()? {
            let category: ProductCategory = ProductCategory::from_row(row, &translate)?;
            return Ok(Some(category));
        }
    }
    Ok(None)
}

pub fn create_category(
    repo: &DuckDbRepository<ProductCategory>,
    category_dto: &ProductCategoryDto,
) -> Result<ProductCategory> {
    let conn = repo.get_connection()?;

    // Create the category with fields from DTO
    let mut category = ProductCategory::new(category_dto.label.clone());
    category.save(&conn)?;
    Ok(category)
}

pub fn update_category(
    repo: &DuckDbRepository<ProductCategory>,
    id: i64,
    category_dto: &ProductCategoryDto,
) -> Result<ProductCategory> {
    let conn = repo.get_connection()?;

    // Check if category exists
    let category_check = get_category_by_id(repo, id)?;
    if category_check.is_none() {
        return Err(custom_error(format!(
            "ProductCategory with id {} not found",
            id
        )));
    }

    let mut category = category_check.unwrap();

    // Update fields
    category.label = category_dto.label.clone();

    category.save(&conn)?;
    Ok(category)
}

// Categories END

// Measurement Unit

pub fn get_measure_unit_with_filter(
    repo: &DuckDbRepository<MeasurementUnit>,
    filters: &FilterOptions,
) -> Result<Vec<MeasurementUnit>> {
    let conn: duckdb::Connection = repo.get_connection()?;

    let mut query = MeasurementUnit::select();

    if let Some(search_term) = &filters.search_term {
        query.filter(
            Internal::Field("title".into()),
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
        query.order("created_at".into(), OrderDirection::DESC);
    }

    // Add pagination
    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.paginate(limit, Some(offset));
    } else if let Some(limit) = filters.limit {
        query.paginate(limit, None);
    }
    let mut units: Vec<MeasurementUnit> = Vec::new();
    let (stmt, translate) = query.all(&conn)?;

    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let unit = MeasurementUnit::from_row(row, &translate);
        if let Ok(unit) = unit {
            if !(units.iter().filter(|&e| e.id == unit.id).count() > 0) {
                units.push(unit);
            }
        }
    }
    Ok(units)
}

pub fn get_measurement_unit_by_id(
    repo: &DuckDbRepository<MeasurementUnit>,
    id: i64,
) -> Result<Option<MeasurementUnit>> {
    let conn = repo.get_connection()?;

    let mut query = MeasurementUnit::select();
    query.filter(
        Internal::Field("id".into()),
        Operations::EqualTo,
        Internal::Value(id.into()),
    );

    let (stmt, translate) = query.all(&conn)?;
    let mut rows = stmt.raw_query();
    while let Some(row) = rows.next()? {
        let unit: MeasurementUnit = MeasurementUnit::from_row(row, &translate)?;
        return Ok(Some(unit));
    }
    // If no rows were found, return None
    Ok(None)
}

pub fn create_measurement_unit(
    repo: &DuckDbRepository<MeasurementUnit>,
    unit_dto: &MeasurementUnitDto,
) -> Result<MeasurementUnit> {
    let conn = repo.get_connection()?;

    // Create the unit
    let mut unit = MeasurementUnit::new(unit_dto.title.clone());

    // Insert the unit
    unit.save(&conn)?;
    Ok(unit)
}

pub fn update_measurement_unit(
    repo: &DuckDbRepository<MeasurementUnit>,
    id: i64,
    unit_dto: &MeasurementUnitDto,
) -> Result<MeasurementUnit> {
    let conn = repo.get_connection()?;

    // Check if unit exists
    let unit_check = get_measurement_unit_by_id(repo, id)?;
    if unit_check.is_none() {
        return Err(custom_error(format!(
            "Measurement unit with id {} not found",
            id
        )));
    }

    let mut unit = unit_check.unwrap();

    // Update fields
    unit.title = unit_dto.title.clone();
    unit.save(&conn)?;
    Ok(unit)
}
