use serde::{Deserialize, Serialize};

use super::model::WarehouseColor;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarehouseDto {
    pub id: Option<i64>,
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColor>,
}
