use serde::{Deserialize, Serialize};
use surrealdb::Datetime;

use crate::warehouse::inputs::WarehouseDto;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategoryDto {
    label: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductWarehouseDto {
    quantity: i64,
    warehouse: Option<WarehouseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnitDto {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDto {
    title: String,
    code: String,
    unit: Option<MeasurementUnitDto>,
    base_price: i64,
    inventory: i64,
    initial_inventory: i64,
    status: String,
    image: String,
    updated_at: Option<Datetime>,
    ware_houses: Vec<ProductWarehouseDto>,
    category: Option<ProductCategoryDto>,
}
