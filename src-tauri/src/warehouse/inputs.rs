use serde::{Deserialize, Serialize};

use super::model::WarehouseColor;

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseDto {
    pub id: Option<String>,
    pub name: String,
    pub shorthand: String,
    pub color: Option<WarehouseColor>,
}
