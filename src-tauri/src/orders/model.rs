use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRole {
    name: String,
    person: Option<Person>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMixedTarget {
    Warehouse(Warehouse),
    Person(Person),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    id: RecordId,
    order_type: String,
    description: String,
    citation_number: String,
    document_date: String,
    document_number: String,
    status: String,

    goods: Vec<OrderProduct>,
    delivery: Option<OrderMixedTarget>,
    recipient: Option<OrderMixedTarget>,
    approvers: Vec<OrderRole>,
    manager: Option<Person>,
    users: Vec<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProduct {
    id: RecordId,
    order: Option<Order>,
    quantity: i64,
}

// `type` Change to `order_type`
