use crate::location::inputs::LocationDto;
use crate::person::inputs::PersonDto;
use crate::warehouse::inputs::WarehouseDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRoleDto {
    pub name: String,
    pub person: Option<PersonDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMixedTargetDto {
    Warehouse(WarehouseDto),
    Person(PersonDto),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderDto {
    pub order_type: String,
    pub description: String,
    pub citation_number: String,
    pub document_date: String,
    pub document_number: String,
    pub status: String,

    pub goods: Vec<OrderProductDto>,
    pub delivery: Option<OrderMixedTargetDto>,
    pub recipient: Option<OrderMixedTargetDto>,
    pub approvers: Vec<OrderRoleDto>,
    pub manager: Option<PersonDto>,
    pub users: Vec<LocationDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProductDto {
    pub order: Option<OrderDto>,
    pub quantity: i64,
}
