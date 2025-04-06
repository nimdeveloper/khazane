use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::warehouse::inputs::WarehouseDto;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategoryDto {
    pub id: Option<String>,
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductWarehouseDto {
    pub quantity: i64,
    pub warehouse: Option<WarehouseDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnitDto {
    pub id: Option<String>,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDto {
    pub id: Option<String>,
    pub title: String,
    pub code: String,
    pub unit: Option<MeasurementUnitDto>,
    pub base_price: i64,
    pub inventory: i64,
    pub initial_inventory: i64,
    pub status: String,
    pub image: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub ware_houses: Vec<ProductWarehouseDto>,
    pub category: Option<ProductCategoryDto>,
}
