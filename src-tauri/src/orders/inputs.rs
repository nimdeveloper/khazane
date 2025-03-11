use crate::location::inputs::LocationDto;
use crate::person::inputs::PersonDto;
use crate::warehouse::inputs::WarehouseDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRoleDto {
    name: String,
    person: Option<PersonDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMixedTargetDto {
    Warehouse(WarehouseDto),
    Person(PersonDto),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderDto {
    order_type: String,
    description: String,
    citation_number: String,
    document_date: String,
    document_number: String,
    status: String,

    goods: Vec<OrderProductDto>,
    delivery: Option<OrderMixedTargetDto>,
    recipient: Option<OrderMixedTargetDto>,
    approvers: Vec<OrderRoleDto>,
    manager: Option<PersonDto>,
    users: Vec<LocationDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProductDto {
    order: Option<OrderDto>,
    quantity: i64,
}
