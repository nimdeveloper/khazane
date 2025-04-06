use crate::core::repository::Model;
use crate::warehouse::model::Warehouse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategory {
    pub id: String,
    pub label: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Model for ProductCategory {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "category"
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductWarehouse {
    pub quantity: i64,
    pub warehouse: Option<Warehouse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnit {
    pub id: String,
    pub title: String,
}

impl Model for MeasurementUnit {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "measure_unit"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub code: String,
    pub unit: Option<MeasurementUnit>,
    pub base_price: i64,
    pub inventory: i64,
    pub initial_inventory: i64,
    pub status: String,
    pub image: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub ware_houses: Vec<ProductWarehouse>,
    pub category: Option<ProductCategory>,
}

impl Model for Product {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "product"
    }
}
