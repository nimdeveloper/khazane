use crate::core::repository::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseColor {
    pub key: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: String,
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColor>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Model for Warehouse {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "warehouse"
    }
}
