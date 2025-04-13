use crate::core::error::Result;
use crate::core::repository::Repository;
use chrono::Utc;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

use super::inputs::{OrderDto, OrderMixedTargetDto, OrderProductDto, OrderRoleDto};
use super::model::{Order, OrderMixedTarget, OrderProduct, OrderRole};
use crate::location::model::Location;
use crate::person::model::Person;
use crate::warehouse::model::Warehouse;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_type: Option<String>,
    pub status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_term: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

pub fn get_order_with_filter(
    repo: &impl Repository<Order>,
    filters: &FilterOptions,
) -> Result<Vec<Order>> {
    let conn = repo.get_connection()?;

    let mut query = String::from(format!(
        "
        SELECT
            o.id,
            o.order_type,
            o.description,
            o.citation_number,
            o.document_date, 
            o.document_number,
            o.status,
            o.created_at,
            o.updated_at,

            o.delivery_type,
            o.delivery_id,

            o.recipient_type, 
            o.recipient_id,

            o.manager_id,

            m.first_name AS manager_first_name,
            m.last_name AS manager_last_name,
            m.national_code AS manager_national_code,
            m.phone AS manager_phone, 
            m.email AS manager_email,
            m.address AS manager_address,
            m.created_at AS manager_created_at,
            m.updated_at AS manager_updated_at

            orl.id AS approver_role_id,
            orl.name AS approver_role_name,
            
            orp.id AS order_product_id,
            orp.quantity AS order_product_quantity,

            orp.product_id AS order_product_product_id,
            orpp.title AS order_product_product_title,
            orpp.code AS order_product_product_code,
            orpp.base_price AS order_product_product_base_price,
            orpp.inventory AS order_product_product_inventory,
            orpp.initial_inventory AS order_product_product_initial_inventory,
            orpp.status AS order_product_product_status,
            orpp.image AS order_product_product_image,
            orpp.created_at AS order_product_product_created_at,
            orpp.updated_at AS order_product_product_updated_at,

            orpp.category_id AS order_product_product_category_id,
            orppc.label AS order_product_product_category_label,
            orppc.created_at AS order_product_product_category_created_at,
            orppc.updated_at AS order_product_product_category_updated_at,

            orpp.unit_id AS order_product_product_unit_id,
            orppu.title AS order_product_product_unit_title,
            orppu.created_at AS order_product_product_unit_created_at,
            orppu.updated_at AS order_product_product_unit_updated_at,

            orppw.id AS order_product_product_warehouse_id,
            orppw.quantity AS order_product_product_warehouse_quantity,
            orppww.id AS order_product_product_warehouse_warehouse_id,
            orppww.name AS order_product_product_warehouse_warehouse_name,
            orppww.shorthand AS order_product_product_warehouse_warehouse_shorthand,
            orppww.color_key AS order_product_product_warehouse_warehouse_color_key,
            orppww.color_code AS order_product_product_warehouse_warehouse_color_code,
            orppww.created_at AS order_product_product_warehouse_warehouse_created_at,
            orppww.updated_at AS order_product_product_warehouse_warehouse_updated_at,

            orlp.id AS approver_person_id,
            orlp.first_name AS approver_person_first_name,
            orlp.last_name AS approver_person_last_name,
            orlp.national_code AS approver_person_national_code,
            orlp.phone AS approver_person_phone,
            orlp.email AS approver_person_email,
            orlp.address AS approver_person_address,
            orlp.created_at AS approver_person_created_at,
            orlp.updated_at AS approver_person_updated_at,

            ou.id AS order_user_id,
            ou.location_id AS order_user_location_id,
            oul.name AS order_user_location_name,
            oul.created_at AS order_user_location_created_at,
            oul.updated_at AS order_user_location_updated_at
        FROM order_table o
        LEFT JOIN person m ON o.manager_id = m.id
        
        LEFT JOIN order_role orl ON orl.order_id = o.id
        LEFT JOIN person orlp ON orl.person_id = person.id
        
        LEFT JOIN order_product orp ON orp.order_id = o.id
        LEFT JOIN product orpp ON orpp.id = orp.product_id
        lEFT JOIN measure_unit orppu ON orppu.id = orpp.unit_id
        LEFT JOIN category orppc ON orppc.id = orpp.category_id
        LEFT JOIN product_warehouse orppw ON orppw.product_id = orpp.id
        LEFT JOIN warehouse orppww ON orppww.id = orppw.warehouse_id

        LEFT JOIN order_user ou ON ou.order_id = o.id
        LEFT JOIN location oul ON l.id = ou.location_id

        WHERE 1=1
    "
    ));

    let mut params: Vec<Box<Order>> = Vec::new();

    if let Some(order_type) = &filters.order_type {
        query.push_str(" AND o.order_type = ?");
        params.push(Box::new(order_type.clone()));
    }

    if let Some(status) = &filters.status {
        query.push_str(" AND o.status = ?");
        params.push(Box::new(status.clone()));
    }

    if let Some(search_term) = &filters.search_term {
        query.push_str(" AND (o.citation_number LIKE ? OR o.document_number LIKE ?)");
        let search_pattern = format!("%{}%", search_term);
        params.push(Box::new(search_pattern.clone()));
        params.push(Box::new(search_pattern));
    }

    if let Some(date_from) = &filters.date_from {
        query.push_str(" AND o.document_date >= ?");
        params.push(Box::new(date_from.clone()));
    }

    if let Some(date_to) = &filters.date_to {
        query.push_str(" AND o.document_date <= ?");
        params.push(Box::new(date_to.clone()));
    }

    // Add sorting
    if let (Some(sort_by), Some(sort_order)) = (&filters.sort_by, &filters.sort_order) {
        query.push_str(&format!(" ORDER BY o.{} {}", sort_by, sort_order));
    } else {
        query.push_str(" ORDER BY o.document_date DESC");
    }

    // Add pagination
    if let (Some(limit), Some(offset)) = (filters.limit, filters.offset) {
        query.push_str(" LIMIT ? OFFSET ?");
        params.push(Box::new(limit));
        params.push(Box::new(offset));
    } else if let Some(limit) = filters.limit {
        query.push_str(" LIMIT ?");
        params.push(Box::new(limit));
    }

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(
        params
            .into_iter()
            .map(|p| p.as_ref())
            .collect::<Vec<_>>()
            .as_slice(),
    )?;
    let mut orders = HashMap::<i64, &Order>::new();
    while let Some(row) = rows.next()? {
        let order_id = row.get(stmt.column_index("id")?)?;
        let mut order: Order;
        if !orders.contains_key(order_id) {
            order = Order::from_row(row, &stmt);
        } else {
            order = orders.get_mut(order_id).unwrap();
        }
        order.set_manager(row, &stmt);
        order.set_recipient(row, &stmt);
        order.set_delivery(row, &stmt);
        order.add_good(row, &stmt);

        // TODO:
        /*
            delivery_type
            delivery_id
            recipient_type
            recipient_id
        */
        // Load goods (order products) for this order
        let goods_query = "
            SELECT op.id, op.order_id, op.product_id, op.quantity, op.created_at, op.updated_at
            FROM order_product op
            WHERE op.order_id = ?
        ";

        let mut goods_stmt = conn.prepare(goods_query)?;
        let mut goods_rows = goods_stmt.query(&[&order.id])?;

        let mut goods = Vec::new();
        while let Some(goods_row) = goods_rows.next()? {
            let product = OrderProduct {
                id: goods_row.get(0)?,
                order: None, // We'll set this later if needed
                quantity: goods_row.get(3)?,
            };
            goods.push(product);
        }

        order.goods = goods;

        // Load approvers for this order
        let approvers_query = "
            SELECT oa.id, oa.order_id, oa.role_id, oa.created_at, oa.updated_at,
                   r.id AS role_id, r.name AS role_name, r.person_id,
                   p.id AS person_id, p.first_name, p.last_name, p.national_code, p.phone, p.email, p.address
            FROM order_approver oa
            JOIN order_role r ON oa.role_id = r.id
            LEFT JOIN person p ON r.person_id = p.id
            WHERE oa.order_id = ?
        ";

        let mut approvers_stmt = conn.prepare(approvers_query)?;
        let mut approvers_rows = approvers_stmt.query(&[&order.id])?;

        let mut approvers = Vec::new();
        while let Some(approver_row) = approvers_rows.next()? {
            let mut approver = OrderRole {
                id: approver_row.get(0)?,
                name: approver_row.get(6)?,
                person: None,
            };

            if let Ok(person_id) = approver_row.get::<_, Option<String>>(8) {
                if let Some(person_id) = person_id {
                    approver.person = Some(Person {
                        id: person_id,
                        first_name: approver_row.get(9)?,
                        last_name: approver_row.get(10)?,
                        national_code: approver_row.get(11)?,
                        phone: approver_row.get(12)?,
                        email: approver_row.get(13)?,
                        address: approver_row.get(14)?,
                        created_at: None,
                        updated_at: None,
                    });
                }
            }

            approvers.push(approver);
        }

        order.approvers = approvers;

        // Load locations for this order
        let locations_query = "
            SELECT ou.id, ou.order_id, ou.location_id, ou.created_at, ou.updated_at,
                   l.id AS location_id, l.name, l.description, l.address, l.location_id AS parent_location_id
            FROM order_user ou
            JOIN location l ON ou.location_id = l.id
            WHERE ou.order_id = ?
        ";

        let mut locations_stmt = conn.prepare(locations_query)?;
        let mut locations_rows = locations_stmt.query(&[&order.id])?;

        let mut locations = Vec::new();
        while let Some(location_row) = locations_rows.next()? {
            let location = Location {
                id: location_row.get(5)?,
                name: location_row.get(6)?,
                description: location_row.get(7)?,
                address: location_row.get(8)?,
                location_id: location_row.get(9)?,
                created_at: None,
                updated_at: None,
            };
            locations.push(location);
        }

        order.users = locations;

        // Load delivery data if needed
        if let Ok(delivery_type) = row.get::<_, Option<String>>(7) {
            if let Some(delivery_type) = delivery_type {
                if let Ok(delivery_id) = row.get::<_, Option<String>>(8) {
                    if let Some(delivery_id) = delivery_id {
                        match delivery_type.as_str() {
                            "warehouse" => {
                                let warehouse_query = "SELECT id, name, shorthand, color_key, color_code FROM warehouse WHERE id = ?";
                                let mut warehouse_stmt = conn.prepare(warehouse_query)?;
                                if let Some(warehouse_row) =
                                    warehouse_stmt.query(&[&delivery_id])?.next()?
                                {
                                    let warehouse = Warehouse {
                                        id: warehouse_row.get(0)?,
                                        name: warehouse_row.get(1)?,
                                        shorthand: warehouse_row.get(2)?,
                                        color_key: warehouse_row.get(3)?,
                                        color_code: warehouse_row.get(4)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.delivery = Some(OrderMixedTarget::Warehouse(warehouse));
                                }
                            }
                            "person" => {
                                let person_query = "SELECT id, first_name, last_name, national_code, phone, email, address FROM person WHERE id = ?";
                                let mut person_stmt = conn.prepare(person_query)?;
                                if let Some(person_row) =
                                    person_stmt.query(&[&delivery_id])?.next()?
                                {
                                    let person = Person {
                                        id: person_row.get(0)?,
                                        first_name: person_row.get(1)?,
                                        last_name: person_row.get(2)?,
                                        national_code: person_row.get(3)?,
                                        phone: person_row.get(4)?,
                                        email: person_row.get(5)?,
                                        address: person_row.get(6)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.delivery = Some(OrderMixedTarget::Person(person));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Load recipient data if needed
        if let Ok(recipient_type) = row.get::<_, Option<String>>(9) {
            if let Some(recipient_type) = recipient_type {
                if let Ok(recipient_id) = row.get::<_, Option<String>>(10) {
                    if let Some(recipient_id) = recipient_id {
                        match recipient_type.as_str() {
                            "warehouse" => {
                                let warehouse_query = "SELECT id, name, shorthand, color_key, color_code FROM warehouse WHERE id = ?";
                                let mut warehouse_stmt = conn.prepare(warehouse_query)?;
                                if let Some(warehouse_row) =
                                    warehouse_stmt.query(&[&recipient_id])?.next()?
                                {
                                    let warehouse = Warehouse {
                                        id: warehouse_row.get(0)?,
                                        name: warehouse_row.get(1)?,
                                        shorthand: warehouse_row.get(2)?,
                                        color_key: warehouse_row.get(3)?,
                                        color_code: warehouse_row.get(4)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.recipient = Some(OrderMixedTarget::Warehouse(warehouse));
                                }
                            }
                            "person" => {
                                let person_query = "SELECT id, first_name, last_name, national_code, phone, email, address FROM person WHERE id = ?";
                                let mut person_stmt = conn.prepare(person_query)?;
                                if let Some(person_row) =
                                    person_stmt.query(&[&recipient_id])?.next()?
                                {
                                    let person = Person {
                                        id: person_row.get(0)?,
                                        first_name: person_row.get(1)?,
                                        last_name: person_row.get(2)?,
                                        national_code: person_row.get(3)?,
                                        phone: person_row.get(4)?,
                                        email: person_row.get(5)?,
                                        address: person_row.get(6)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.recipient = Some(OrderMixedTarget::Person(person));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        orders.push(order);
    }

    Ok(orders)
}

pub fn get_order_by_id(
    repo: &impl Repository<Order>,
    id: &str,
) -> Result<Option<Order>, Box<dyn Error>> {
    let conn = repo.get_connection()?;

    let query = "
        SELECT o.id, o.order_type, o.description, o.citation_number, o.document_date, 
               o.document_number, o.status, o.delivery_type, o.delivery_id, o.recipient_type, 
               o.recipient_id, o.manager_id, o.created_at, o.updated_at,
               m.id AS manager_id, m.first_name AS manager_first_name, m.last_name AS manager_last_name,
               m.national_code AS manager_national_code, m.phone AS manager_phone, 
               m.email AS manager_email, m.address AS manager_address
        FROM order_table o
        LEFT JOIN person m ON o.manager_id = m.id
        WHERE o.id = ?
    ";

    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query(&[id])?;

    if let Some(row) = rows.next()? {
        let mut order = Order {
            id: row.get(0)?,
            order_type: row.get(1)?,
            description: row.get(2)?,
            citation_number: row.get(3)?,
            document_date: row.get(4)?,
            document_number: row.get(5)?,
            status: row.get(6)?,
            delivery: None,
            recipient: None,
            goods: Vec::new(),
            approvers: Vec::new(),
            manager: None,
            users: Vec::new(),
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        };

        // Parse manager if available
        if let Ok(manager_id) = row.get::<_, Option<String>>(14) {
            if let Some(manager_id) = manager_id {
                order.manager = Some(Person {
                    id: manager_id,
                    first_name: row.get(15)?,
                    last_name: row.get(16)?,
                    national_code: row.get(17)?,
                    phone: row.get(18)?,
                    email: row.get(19)?,
                    address: row.get(20)?,
                    created_at: None,
                    updated_at: None,
                });
            }
        }

        // Load goods (order products) for this order
        let goods_query = "
            SELECT op.id, op.order_id, op.product_id, op.quantity, op.created_at, op.updated_at
            FROM order_product op
            WHERE op.order_id = ?
        ";

        let mut goods_stmt = conn.prepare(goods_query)?;
        let mut goods_rows = goods_stmt.query(&[&order.id])?;

        let mut goods = Vec::new();
        while let Some(goods_row) = goods_rows.next()? {
            let product = OrderProduct {
                id: goods_row.get(0)?,
                order: None, // We'll set this later if needed
                quantity: goods_row.get(3)?,
            };
            goods.push(product);
        }

        order.goods = goods;

        // Load approvers for this order
        let approvers_query = "
            SELECT oa.id, oa.order_id, oa.role_id, oa.created_at, oa.updated_at,
                   r.id AS role_id, r.name AS role_name, r.person_id,
                   p.id AS person_id, p.first_name, p.last_name, p.national_code, p.phone, p.email, p.address
            FROM order_approver oa
            JOIN order_role r ON oa.role_id = r.id
            LEFT JOIN person p ON r.person_id = p.id
            WHERE oa.order_id = ?
        ";

        let mut approvers_stmt = conn.prepare(approvers_query)?;
        let mut approvers_rows = approvers_stmt.query(&[&order.id])?;

        let mut approvers = Vec::new();
        while let Some(approver_row) = approvers_rows.next()? {
            let mut approver = OrderRole {
                id: approver_row.get(0)?,
                name: approver_row.get(6)?,
                person: None,
            };

            if let Ok(person_id) = approver_row.get::<_, Option<String>>(8) {
                if let Some(person_id) = person_id {
                    approver.person = Some(Person {
                        id: person_id,
                        first_name: approver_row.get(9)?,
                        last_name: approver_row.get(10)?,
                        national_code: approver_row.get(11)?,
                        phone: approver_row.get(12)?,
                        email: approver_row.get(13)?,
                        address: approver_row.get(14)?,
                        created_at: None,
                        updated_at: None,
                    });
                }
            }

            approvers.push(approver);
        }

        order.approvers = approvers;

        // Load locations for this order
        let locations_query = "
            SELECT ou.id, ou.order_id, ou.location_id, ou.created_at, ou.updated_at,
                   l.id AS location_id, l.name, l.description, l.address, l.location_id AS parent_location_id
            FROM order_user ou
            JOIN location l ON ou.location_id = l.id
            WHERE ou.order_id = ?
        ";

        let mut locations_stmt = conn.prepare(locations_query)?;
        let mut locations_rows = locations_stmt.query(&[&order.id])?;

        let mut locations = Vec::new();
        while let Some(location_row) = locations_rows.next()? {
            let location = Location {
                id: location_row.get(5)?,
                name: location_row.get(6)?,
                description: location_row.get(7)?,
                address: location_row.get(8)?,
                location_id: location_row.get(9)?,
                created_at: None,
                updated_at: None,
            };
            locations.push(location);
        }

        order.users = locations;

        // Load delivery data if needed
        if let Ok(delivery_type) = row.get::<_, Option<String>>(7) {
            if let Some(delivery_type) = delivery_type {
                if let Ok(delivery_id) = row.get::<_, Option<String>>(8) {
                    if let Some(delivery_id) = delivery_id {
                        match delivery_type.as_str() {
                            "warehouse" => {
                                let warehouse_query = "SELECT id, name, shorthand, color_key, color_code FROM warehouse WHERE id = ?";
                                let mut warehouse_stmt = conn.prepare(warehouse_query)?;
                                if let Some(warehouse_row) =
                                    warehouse_stmt.query(&[&delivery_id])?.next()?
                                {
                                    let warehouse = Warehouse {
                                        id: warehouse_row.get(0)?,
                                        name: warehouse_row.get(1)?,
                                        shorthand: warehouse_row.get(2)?,
                                        color_key: warehouse_row.get(3)?,
                                        color_code: warehouse_row.get(4)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.delivery = Some(OrderMixedTarget::Warehouse(warehouse));
                                }
                            }
                            "person" => {
                                let person_query = "SELECT id, first_name, last_name, national_code, phone, email, address FROM person WHERE id = ?";
                                let mut person_stmt = conn.prepare(person_query)?;
                                if let Some(person_row) =
                                    person_stmt.query(&[&delivery_id])?.next()?
                                {
                                    let person = Person {
                                        id: person_row.get(0)?,
                                        first_name: person_row.get(1)?,
                                        last_name: person_row.get(2)?,
                                        national_code: person_row.get(3)?,
                                        phone: person_row.get(4)?,
                                        email: person_row.get(5)?,
                                        address: person_row.get(6)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.delivery = Some(OrderMixedTarget::Person(person));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Load recipient data if needed
        if let Ok(recipient_type) = row.get::<_, Option<String>>(9) {
            if let Some(recipient_type) = recipient_type {
                if let Ok(recipient_id) = row.get::<_, Option<String>>(10) {
                    if let Some(recipient_id) = recipient_id {
                        match recipient_type.as_str() {
                            "warehouse" => {
                                let warehouse_query = "SELECT id, name, shorthand, color_key, color_code FROM warehouse WHERE id = ?";
                                let mut warehouse_stmt = conn.prepare(warehouse_query)?;
                                if let Some(warehouse_row) =
                                    warehouse_stmt.query(&[&recipient_id])?.next()?
                                {
                                    let warehouse = Warehouse {
                                        id: warehouse_row.get(0)?,
                                        name: warehouse_row.get(1)?,
                                        shorthand: warehouse_row.get(2)?,
                                        color_key: warehouse_row.get(3)?,
                                        color_code: warehouse_row.get(4)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.recipient = Some(OrderMixedTarget::Warehouse(warehouse));
                                }
                            }
                            "person" => {
                                let person_query = "SELECT id, first_name, last_name, national_code, phone, email, address FROM person WHERE id = ?";
                                let mut person_stmt = conn.prepare(person_query)?;
                                if let Some(person_row) =
                                    person_stmt.query(&[&recipient_id])?.next()?
                                {
                                    let person = Person {
                                        id: person_row.get(0)?,
                                        first_name: person_row.get(1)?,
                                        last_name: person_row.get(2)?,
                                        national_code: person_row.get(3)?,
                                        phone: person_row.get(4)?,
                                        email: person_row.get(5)?,
                                        address: person_row.get(6)?,
                                        created_at: None,
                                        updated_at: None,
                                    };
                                    order.recipient = Some(OrderMixedTarget::Person(person));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        return Ok(Some(order));
    }

    Ok(None)
}

pub fn create_order(
    repo: &impl Repository<Order>,
    order_dto: &OrderDto,
) -> Result<Order, Box<dyn Error>> {
    let conn = repo.get_connection()?;

    // Generate new UUID
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    // Extract delivery and recipient information
    let (delivery_type, delivery_id) = match &order_dto.delivery {
        Some(OrderMixedTargetDto::Warehouse(w)) => {
            (Some("warehouse".to_string()), Some(w.id.clone().unwrap()))
        }
        Some(OrderMixedTargetDto::Person(p)) => {
            (Some("person".to_string()), Some(p.id.clone().unwrap()))
        }
        None => (None, None),
    };

    let (recipient_type, recipient_id) = match &order_dto.recipient {
        Some(OrderMixedTargetDto::Warehouse(w)) => {
            (Some("warehouse".to_string()), Some(w.id.clone().unwrap()))
        }
        Some(OrderMixedTargetDto::Person(p)) => {
            (Some("person".to_string()), Some(p.id.clone().unwrap()))
        }
        None => (None, None),
    };

    // Create the order
    let order = Order {
        id: id.clone(),
        order_type: order_dto.order_type.clone(),
        description: order_dto.description.clone(),
        citation_number: order_dto.citation_number.clone(),
        document_date: order_dto.document_date.clone(),
        document_number: order_dto.document_number.clone(),
        status: order_dto.status.clone(),
        delivery: None,        // Will be set after creation
        recipient: None,       // Will be set after creation
        goods: Vec::new(),     // Will be set after creation
        approvers: Vec::new(), // Will be set after creation
        manager: None,         // Will be set after creation
        users: Vec::new(),     // Will be set after creation
        created_at: Some(now),
        updated_at: Some(now),
    };

    // Insert into the database
    let insert_sql = "
        INSERT INTO order_table (
            id, order_type, description, citation_number, document_date, 
            document_number, status, delivery_type, delivery_id, recipient_type, 
            recipient_id, manager_id, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ";

    let mut stmt = conn.prepare(insert_sql)?;
    stmt.execute(&[
        &order.id,
        &order.order_type,
        &order.description,
        &order.citation_number,
        &order.document_date,
        &order.document_number,
        &order.status,
        &delivery_type,
        &delivery_id,
        &recipient_type,
        &recipient_id,
        &order_dto.manager.as_ref().and_then(|m| m.id.clone()),
        &order.created_at,
        &order.updated_at,
    ])?;

    // Create order products
    for product in &order_dto.goods {
        let product_id = Uuid::new_v4().to_string();
        let insert_product_sql = "
            INSERT INTO order_product (id, order_id, product_id, quantity, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
        ";

        let mut product_stmt = conn.prepare(insert_product_sql)?;
        product_stmt.execute(&[
            &product_id,
            &order.id,
            &product.order.as_ref().and_then(|o| o.id.clone()).unwrap(),
            &product.quantity,
            &now,
            &now,
        ])?;
    }

    // Create order approvers
    for approver in &order_dto.approvers {
        let approver_id = Uuid::new_v4().to_string();
        let insert_approver_sql = "
            INSERT INTO order_approver (id, order_id, role_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
        ";

        let mut approver_stmt = conn.prepare(insert_approver_sql)?;
        approver_stmt.execute(&[
            &approver_id,
            &order.id,
            &approver.id.clone().unwrap(),
            &now,
            &now,
        ])?;
    }

    // Create order users (locations)
    for user in &order_dto.users {
        let user_id = Uuid::new_v4().to_string();
        let insert_user_sql = "
            INSERT INTO order_user (id, order_id, location_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
        ";

        let mut user_stmt = conn.prepare(insert_user_sql)?;
        user_stmt.execute(&[&user_id, &order.id, &user.id.clone().unwrap(), &now, &now])?;
    }

    // Get the complete order with all relationships
    let complete_order = get_order_by_id(repo, &order.id)?.unwrap();

    Ok(complete_order)
}

pub fn update_order(
    repo: &impl Repository<Order>,
    id: &str,
    order_dto: &OrderDto,
) -> Result<Order, Box<dyn Error>> {
    let conn = repo.get_connection()?;

    // Check if order exists
    let order_check = get_order_by_id(repo, id)?;
    if order_check.is_none() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Order with id {} not found", id),
        )));
    }

    let now = Utc::now();

    // Extract delivery and recipient information
    let (delivery_type, delivery_id) = match &order_dto.delivery {
        Some(OrderMixedTargetDto::Warehouse(w)) => {
            (Some("warehouse".to_string()), Some(w.id.clone().unwrap()))
        }
        Some(OrderMixedTargetDto::Person(p)) => {
            (Some("person".to_string()), Some(p.id.clone().unwrap()))
        }
        None => (None, None),
    };

    let (recipient_type, recipient_id) = match &order_dto.recipient {
        Some(OrderMixedTargetDto::Warehouse(w)) => {
            (Some("warehouse".to_string()), Some(w.id.clone().unwrap()))
        }
        Some(OrderMixedTargetDto::Person(p)) => {
            (Some("person".to_string()), Some(p.id.clone().unwrap()))
        }
        None => (None, None),
    };

    // Update the order
    let update_sql = "
        UPDATE order_table 
        SET order_type = ?, description = ?, citation_number = ?, document_date = ?, 
            document_number = ?, status = ?, delivery_type = ?, delivery_id = ?, 
            recipient_type = ?, recipient_id = ?, manager_id = ?, updated_at = ?
        WHERE id = ?
    ";

    let mut stmt = conn.prepare(update_sql)?;
    stmt.execute(&[
        &order_dto.order_type,
        &order_dto.description,
        &order_dto.citation_number,
        &order_dto.document_date,
        &order_dto.document_number,
        &order_dto.status,
        &delivery_type,
        &delivery_id,
        &recipient_type,
        &recipient_id,
        &order_dto.manager.as_ref().and_then(|m| m.id.clone()),
        &now,
        id,
    ])?;

    // Delete existing order products
    let delete_products_sql = "DELETE FROM order_product WHERE order_id = ?";
    let mut delete_products_stmt = conn.prepare(delete_products_sql)?;
    delete_products_stmt.execute(&[id])?;

    // Create new order products
    for product in &order_dto.goods {
        let product_id = Uuid::new_v4().to_string();
        let insert_product_sql = "
            INSERT INTO order_product (id, order_id, product_id, quantity, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
        ";

        let mut product_stmt = conn.prepare(insert_product_sql)?;
        product_stmt.execute(&[
            &product_id,
            id,
            &product.order.as_ref().and_then(|o| o.id.clone()).unwrap(),
            &product.quantity,
            &now,
            &now,
        ])?;
    }

    // Delete existing order approvers
    let delete_approvers_sql = "DELETE FROM order_approver WHERE order_id = ?";
    let mut delete_approvers_stmt = conn.prepare(delete_approvers_sql)?;
    delete_approvers_stmt.execute(&[id])?;

    // Create new order approvers
    for approver in &order_dto.approvers {
        let approver_id = Uuid::new_v4().to_string();
        let insert_approver_sql = "
            INSERT INTO order_approver (id, order_id, role_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
        ";

        let mut approver_stmt = conn.prepare(insert_approver_sql)?;
        approver_stmt.execute(&[&approver_id, id, &approver.id.clone().unwrap(), &now, &now])?;
    }

    // Delete existing order users
    let delete_users_sql = "DELETE FROM order_user WHERE order_id = ?";
    let mut delete_users_stmt = conn.prepare(delete_users_sql)?;
    delete_users_stmt.execute(&[id])?;

    // Create new order users
    for user in &order_dto.users {
        let user_id = Uuid::new_v4().to_string();
        let insert_user_sql = "
            INSERT INTO order_user (id, order_id, location_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
        ";

        let mut user_stmt = conn.prepare(insert_user_sql)?;
        user_stmt.execute(&[&user_id, id, &user.id.clone().unwrap(), &now, &now])?;
    }

    // Get the complete updated order
    let updated_order = get_order_by_id(repo, id)?.unwrap();

    Ok(updated_order)
}
