use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRole {
    pub name: String,
    pub person: Option<Person>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMixedTarget {
    Warehouse(Warehouse),
    Person(Person),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: RecordId,
    pub order_type: String,
    pub description: String,
    pub citation_number: String,
    pub document_date: String,
    pub document_number: String,
    pub status: String,

    pub goods: Vec<OrderProduct>,
    pub delivery: Option<OrderMixedTarget>,
    pub recipient: Option<OrderMixedTarget>,
    pub approvers: Vec<OrderRole>,
    pub manager: Option<Person>,
    pub users: Vec<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProduct {
    pub order: Option<Order>,
    pub quantity: i64,
}

// `type` Change to `order_type`
