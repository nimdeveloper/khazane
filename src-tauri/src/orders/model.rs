use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize)]
struct Order<'a> {
    id: RecordId,
    order_type: &'a str,
    description: &'a str,
    citation_number: &'a str,
    document_date: &'a str,
    document_number: &'a str,
    status: &'a str,

    delivery: Option<Warehouse<'a>>,
    recipient: Option<Warehouse<'a>>,
    manager: Option<Person<'a>>,
    users: Vec<Location<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OrderProduct<'a> {
    id: RecordId,
    order: Option<Order<'a>>,
    quantity: i64,
}

// `type` Change to `order_type`

// delivery: IPerson | IWareHouse | null;
// recipient: IPerson | IWareHouse | null;

// approvers: IRole[];
// goods: IOrderProduct[];
// users: (ILocation | null)[];
