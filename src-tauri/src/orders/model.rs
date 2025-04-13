use std::hash::Hash;
use std::{any, collections::HashMap};

use crate::core::error::Error;
use crate::core::helpers::{self, flatten_with_prefix};
use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;
use crate::{core::repository::Model, product::model::Product};
use duckdb::types::ValueRef;
use duckdb::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderRole {
    pub id: i64,
    pub name: String,
    pub person: Option<Person>,
}
impl OrderRole {
    const MODEL_QUERY_PREFIX: &'static str = "order_role_";
    fn validate_map(m: HashMap<String, any::Any>) {}
    fn from_map(m: HashMap<String, any::Any>, load_relations: bool) -> Self {
        let mut res = Self {
            id: m.get("id")?,
            name: m.get("name")?,
            person: None,
        };
        if load_relations {
            res.set_person(m);
        }
        res
    }
    fn set_person<T>(&self, m: HashMap<String, T>) -> Result<Self, Error> {
        if Some(_) = self.person {
            return;
        }
        let temp_person = Person::from_map(helpers::flatten_with_prefix(
            Person::MODEL_QUERY_PREFIX.to_string(),
            m,
        ));
        if let Ok(temp_person) = temp_person {
            self.person = Some(temp_person);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderMixedTarget {
    Warehouse(Warehouse),
    Person(Person),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub id: i64,
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
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Order {
    const MODEL_QUERY_PREFIX: &'static str = "order_";
    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Self {
        Order {
            id: row.get(stmt.column_index("id")?)?,
            order_type: row.get(stmt.column_index("order_type")?)?,
            description: row.get(stmt.column_index("description")?)?,
            citation_number: row.get(stmt.column_index("citation_number")?)?,
            document_date: row.get(stmt.column_index("document_date")?)?,
            document_number: row.get(stmt.column_index("document_number")?)?,
            status: row.get(stmt.column_index("status")?)?,
            created_at: row.get(stmt.column_index("created_at")?)?,
            updated_at: row.get(stmt.column_index("updated_at")?)?,

            goods: Vec::new(),
            delivery: None,
            recipient: None,
            approvers: Vec::new(),
            manager: None,
            users: Vec::new(),
        }
    }
    pub fn set_delivery(&self, m: HashMap<String, ValueRef>) {}
    pub fn set_recipient(&self, m: HashMap<String, ValueRef>) {}
    pub fn set_manager(&self, m: HashMap<String, ValueRef>) {
        if let Some(_) = self.manager {
            return;
        }

        let temp_person = Person::from_map(helpers::flatten_with_prefix(
            Person::MODEL_QUERY_PREFIX.to_string(),
            m,
        ));
        if let Ok(temp_person) = temp_person {
            self.manager = Some(temp_person);
        }
    }
    pub fn add_good(&self, m: HashMap<String, ValueRef>) {
        let goods_map: HashMap<String, ValueRef> =
            flatten_with_prefix(OrderProduct::MODEL_QUERY_PREFIX.to_string(), m);
        let mut good_id: Option<String> = goods_map.get("id");
        let mut the_good: Option<OrderProduct> = None;

        if let Some(good_id) = good_id {
            let old_goods: Vec<OrderProduct> = self.goods.iter().filter(|&each| each.id == good_id);
            if old_goods.iter().count() > 0 {
                the_good = old_goods.get(0);
            }
        }
        if let None = the_good {
            if let Ok(tmp_good) = OrderProduct::from_map(goods_map, false) {
                the_good = Some(tmp_good);
            } else {
                return; // Failed to set approver
            }
            self.approvers.append(the_good.unwrap());
        }
        the_good?.set_product(goods_map);
    }
    pub fn add_approver(&self, m: HashMap<String, ValueRef>) {
        let approvers_map: HashMap<String, ValueRef> =
            flatten_with_prefix(OrderRole::MODEL_QUERY_PREFIX.to_string(), m);
        let mut approver_id: Option<String> = approvers_map.get("id");
        let mut the_approver: Option<OrderRole> = None;

        if let Some(approver_id) = approver_id {
            let old_approvers: Vec<OrderRole> =
                self.approvers.iter().filter(|&each| each.id == approver_id);
            if old_approvers.iter().count() > 0 {
                the_approver = old_approvers.get(0);
            }
        }
        if let None = the_approver {
            if let Ok(tmp_approver) = OrderRole::from_map(approvers_map, false) {
                the_approver = Some(tmp_approver);
            } else {
                return; // Failed to set approver
            }
            self.approvers.append(the_approver.unwrap());
        }
        the_approver?.set_person(approvers_map);
    }
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
    pub id: i64,
    pub product: Option<Product>,
    pub quantity: i64,
}
impl OrderProduct {
    const MODEL_QUERY_PREFIX: &'static str = "order_product_";
    fn validate_map(m: HashMap<String, any::Any>) {}
    fn from_map(m: HashMap<String, any::Any>, load_relations: bool) -> Self {
        let mut res = OrderProduct {
            id: m.get("id"),
            product: None,
            quantity: m.get("quantity"),
        };
        if load_relations {
            res.set_product(m);
        }
        res
    }
    fn set_product<T>(&self, m: HashMap<String, T>) -> Result<Self, Error> {
        if Some(_) = self.product {
            return;
        }
        self.product = Product::from_map(
            helpers::flatten_with_prefix(Product::MODEL_QUERY_PREFIX.to_string(), m),
            true,
        );
    }
}

// `type` Change to `order_type`
