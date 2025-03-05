use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRole<'a> {
    name: &'a str,
    person: Option<Person<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMixedTarget<'a> {
    #[serde(borrow)]
    Warehouse(Warehouse<'a>),
    Person(Person<'a>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order<'a> {
    id: RecordId,
    order_type: &'a str,
    description: &'a str,
    citation_number: &'a str,
    document_date: &'a str,
    document_number: &'a str,
    status: &'a str,

    goods: Vec<OrderProduct<'a>>,
    delivery: Option<OrderMixedTarget<'a>>,
    recipient: Option<OrderMixedTarget<'a>>,
    approvers: Vec<OrderRole<'a>>,
    manager: Option<Person<'a>>,
    users: Vec<Location<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProduct<'a> {
    id: RecordId,
    #[serde(borrow)]
    order: Option<Order<'a>>,
    quantity: i64,
}

// `type` Change to `order_type`
