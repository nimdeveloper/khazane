use crate::core::repository::Model;
use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRole {
    pub id: String,
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
    pub id: String,
    pub order_type: String,
    pub description: String,
    pub citation_number: String,
    pub document_date: String,
    pub document_number: String,
    pub status: String,
    pub goods: Vec<OrderProduct>, // Related using order_product table
    pub delivery: Option<OrderMixedTarget>,
    pub recipient: Option<OrderMixedTarget>,
    pub approvers: Vec<OrderRole>, // Related using order_approver table
    pub manager: Option<Person>,
    pub users: Vec<Location>, // Related using order_user table
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Model for Order {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "order_table" // 'order' is a SQL keyword, so we use order_table
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderProduct {
    pub id: String,
    pub order: Option<Order>,
    pub quantity: i64,
}

// `type` Change to `order_type`
