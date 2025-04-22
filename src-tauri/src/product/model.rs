#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::database::value_ref_to_type;
use crate::core::error::{custom_error, Error, Result};
use crate::core::repository::Model;
use crate::core::selector::{
    Condition, DbTranslateBox, Internal, Join, JoinMethod, Operations, Selector,
};
use crate::warehouse::model::Warehouse;
use chrono::{NaiveDateTime, Utc};
use duckdb::types::ValueRef;
use duckdb::{params_from_iter, Connection, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategory {
    pub id: i64,
    pub label: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl ProductCategory {
    pub fn new(label: String) -> Self {
        ProductCategory {
            id: 0,
            label,
            created_at: None,
            updated_at: None,
        }
    }
    pub fn get_columns() -> [String; 4] {
        [
            "id".to_string(),
            "label".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector
    }
    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        Ok(Self {
            id: row.get(translator.field("id")?)?,
            label: row.get(translator.field("label")?)?,
            created_at: row.get(translator.field("created_at")?)?,
            updated_at: row.get(translator.field("updated_at")?)?,
        })
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self> {
        // if Self::get_columns().iter().any(|e| !m.contains_key(e)) {
        //     return Err(custom_error(
        //         "Failed to construct ProductCategory from HasMap! Some keys missing!",
        //     ));
        // }
        let id: i64 = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from)?;

        let label: String = value_ref_to_type(m.get("label").unwrap()).map_err(Error::from)?;

        let created_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from)?;

        let updated_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from)?;

        Ok(Self {
            id,
            label,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
    pub fn save<'a>(&mut self, connection: &'a Connection) -> Result<()> {
        let mut params: Vec<Rc<dyn ToSql>> = Vec::new();
        let query;
        let now = Utc::now();

        if self.id > 0 {
            self.updated_at = Some(now.naive_utc());
            query = format!(
                "UPDATE {} SET label=? , updated_at=? WHERE id = ?",
                Self::TABLE_NAME
            );
            params.push(Rc::new(self.label.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.id.clone()));
        } else {
            self.created_at = Some(now.naive_utc());
            self.updated_at = Some(now.naive_utc());
            params.push(Rc::new(self.label.clone()));
            params.push(Rc::new(self.created_at.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            query = format!(
                "INSERT INTO {} (label,created_at,updated_at) VALUES (?,?,?) RETURNING id",
                Self::TABLE_NAME
            );
        }
        if self.id > 0 {
            // Update in the database
            let mut stmt = connection.prepare(&query)?;
            let changes = stmt.execute(params_from_iter(params.into_iter()))?;
            if changes <= 0 {
                todo!("Add warning?")
            }
            return Ok(());
        } else {
            let mut stmt = connection.prepare(&query)?;
            stmt.execute(params_from_iter(params.into_iter()))?;
            return match stmt.raw_query().next()? {
                Some(row) => {
                    let id = row.get(stmt.column_index("id")?)?;
                    self.id = id;
                    return Ok(());
                }
                None => Err(custom_error("failed to insert at 'product category'!")),
            };
        }
    }
}
impl Model for ProductCategory {
    const TABLE_NAME: &str = "category";
    fn get_id(&self) -> String {
        self.id.to_string().clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductWarehouse {
    pub id: i64,
    pub quantity: i64,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub product_id: i64,
    pub warehouse: Option<Warehouse>,
}
impl ProductWarehouse {
    pub fn new(quantity: i64, product_id: i64, warehouse: Option<Warehouse>) -> Self {
        ProductWarehouse {
            id: 0,
            quantity,
            product_id,
            warehouse,
        }
    }
    pub fn bulk_insert(values: &Vec<ProductWarehouse>, conn: &Connection) -> Result<()> {
        let mut params: Vec<Rc<dyn ToSql>> = Vec::new();
        let mut query = format!(
            "INSERT INTO {} (quantity, product_id, warehouse_id) VALUES ",
            Self::TABLE_NAME
        );
        for value in values.iter() {
            query.push_str(" (?, ?, ?),");
            params.push(Rc::new(value.quantity.clone()));
            params.push(Rc::new(value.product_id.clone()));
            params.push(Rc::new(
                value
                    .warehouse
                    .as_ref()
                    .map(|w| Some(w.id.clone()))
                    .unwrap_or(None),
            ));
        }
        if query.ends_with(",") {
            query.pop();
        }
        log::trace!("{}", query);

        let mut stmt = conn.prepare(&query)?;
        let changes = stmt.execute(params_from_iter(params.into_iter()))?;
        if changes <= 0 || changes != values.len() {
            custom_error(
                "Failed to insert at 'product warehouse'! No rows inserted or wrong insert count!",
            );
        }
        Ok(())
    }
    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        let mut warehouse = None;
        if let Some(_) = row.get::<_, Option<i64>>(translator.field("warehouse_id")?)? {
            match translator.with_rel("warehouse") {
                Ok(rel) => {
                    warehouse = Some(Warehouse::from_row(row, &rel)?);
                }
                Err(e) => log::error!("{}", e),
            };
        }
        Ok(Self {
            id: row.get(translator.field("id")?)?,
            quantity: row.get(translator.field("quantity")?)?,
            product_id: row.get(translator.field("product_id")?)?,
            warehouse,
        })
    }
    pub fn select() -> Selector {
        let mut s = Selector::default();
        s.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        s.with(
            Join::to(
                Warehouse::TABLE_NAME,
                JoinMethod::LeftJoin,
                Condition::new(
                    Internal::Field("warehouse_id".into()),
                    Operations::EqualTo,
                    Internal::Field("warehouse__id".into()),
                ),
                Warehouse::get_columns().to_vec(),
            ),
            "warehouse",
        );
        s
    }
    // pub fn as_join() {
    //     Join::to(Self::TABLE_NAME, JoinMethod::LeftJoinJoin)
    //         .on("id", "warehouse_id")
    //         .with(Warehouse::TABLE_NAME)
    //         .as("warehouse")
    // }
    pub fn get_columns() -> [String; 4] {
        [
            "id".to_string(),
            "quantity".to_string(),
            "warehouse_id".to_string(),
            "product_id".to_string(),
        ]
    }
}
impl Model for ProductWarehouse {
    const TABLE_NAME: &str = "product_warehouse";
    fn get_id(&self) -> String {
        self.id.to_string().clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnit {
    pub id: i64,
    pub title: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl MeasurementUnit {
    pub fn new(title: String) -> Self {
        Self {
            id: 0,
            title,
            created_at: None,
            updated_at: None,
        }
    }
    pub fn get_columns() -> [String; 4] {
        [
            "id".to_string(),
            "title".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector
    }
    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        Ok(Self {
            id: row.get(translator.field("id")?)?,
            title: row.get(translator.field("title")?)?,
            created_at: row.get(translator.field("created_at")?)?,
            updated_at: row.get(translator.field("updated_at")?)?,
        })
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self> {
        // if Self::get_columns().iter().any(|e| !m.contains_key(e)) {
        //     return Err(custom_error(
        //         "Failed to construct ProductCategory from HasMap! Some keys missing!",
        //     ));
        // }
        let id: i64 = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from)?;

        let title: String = value_ref_to_type(m.get("title").unwrap()).map_err(Error::from)?;

        let created_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from)?;

        let updated_at: chrono::NaiveDateTime =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from)?;

        Ok(Self {
            id,
            title,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
    pub fn save<'a>(&mut self, connection: &'a Connection) -> Result<()> {
        let mut params: Vec<Rc<dyn ToSql>> = Vec::new();
        let query;
        let now = Utc::now();

        if self.id > 0 {
            self.updated_at = Some(now.naive_utc());
            query = format!(
                "UPDATE {} SET title=? , updated_at=? WHERE id = ?",
                Self::TABLE_NAME
            );
            params.push(Rc::new(self.title.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.id.clone()));
        } else {
            self.created_at = Some(now.naive_utc());
            self.updated_at = Some(now.naive_utc());
            params.push(Rc::new(self.title.clone()));
            params.push(Rc::new(self.created_at.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            query = format!(
                "INSERT INTO {} (title,created_at,updated_at) VALUES (?,?,?) RETURNING id",
                Self::TABLE_NAME
            );
        }
        if self.id > 0 {
            // Update in the database
            let mut stmt = connection.prepare(&query)?;
            let changes = stmt.execute(params_from_iter(params.into_iter()))?;
            if changes <= 0 {
                todo!("Add warning?")
            }
            return Ok(());
        } else {
            let mut stmt = connection.prepare(&query)?;
            stmt.execute(params_from_iter(params.into_iter()))?;
            return match stmt.raw_query().next()? {
                Some(row) => {
                    let id = row.get(stmt.column_index("id")?)?;
                    self.id = id;
                    return Ok(());
                }
                None => Err(custom_error("failed to insert at 'product category'!")),
            };
        }
    }
}
impl Model for MeasurementUnit {
    const TABLE_NAME: &str = "measure_unit";
    fn get_id(&self) -> String {
        self.id.to_string().clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub code: String,
    pub base_price: i64,
    pub inventory: i64,
    pub initial_inventory: i64,
    pub status: String,
    pub image: String,
    // Timestamps
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    // Relations
    pub unit: Option<MeasurementUnit>,
    pub ware_houses: Vec<ProductWarehouse>,
    pub category: Option<ProductCategory>,
}
impl Model for Product {
    const TABLE_NAME: &str = "product";
    fn get_id(&self) -> String {
        self.id.to_string().clone()
    }
}

impl Product {
    pub fn new(
        title: String,
        code: String,
        base_price: i64,
        inventory: i64,
        initial_inventory: i64,
        status: String,
        image: String,
    ) -> Self {
        Product {
            id: 0,
            title,
            code,
            unit: None,
            base_price,
            inventory,
            initial_inventory,
            status,
            image,
            created_at: None,
            updated_at: None,
            ware_houses: Vec::<ProductWarehouse>::new(),
            category: None,
        }
    }
    pub fn get_columns() -> [String; 12] {
        [
            "id".to_string(),
            "title".to_string(),
            "code".to_string(),
            "base_price".to_string(),
            "inventory".to_string(),
            "initial_inventory".to_string(),
            "status".to_string(),
            "image".to_string(),
            // Timestamps
            "created_at".to_string(),
            "updated_at".to_string(),
            // Relations
            "category_id".to_string(),
            "unit_id".to_string(),
        ]
    }
    pub fn select() -> Selector {
        let mut selector = Selector::default();
        selector.select(Self::get_columns().to_vec(), Self::TABLE_NAME);
        selector.with(
            Join::to(
                MeasurementUnit::TABLE_NAME,
                JoinMethod::LeftJoin,
                Condition::new(
                    Internal::Field("unit_id".into()),
                    Operations::EqualTo,
                    Internal::Field("unit__id".into()),
                ),
                MeasurementUnit::get_columns().to_vec(),
            ),
            "unit",
        );
        selector.with(
            Join::to(
                ProductCategory::TABLE_NAME,
                JoinMethod::LeftJoin,
                Condition::new(
                    Internal::Field("category_id".into()),
                    Operations::EqualTo,
                    Internal::Field("category__id".into()),
                ),
                ProductCategory::get_columns().to_vec(),
            ),
            "category",
        );
        selector
    }
    pub fn from_row(row: &duckdb::Row, translator: &DbTranslateBox) -> Result<Self> {
        Ok(Self {
            id: row.get(translator.field("id")?)?,
            title: row.get(translator.field("title")?)?,
            code: row.get(translator.field("code")?)?,
            base_price: row.get(translator.field("base_price")?)?,
            inventory: row.get(translator.field("inventory")?)?,
            initial_inventory: row.get(translator.field("initial_inventory")?)?,
            status: row.get(translator.field("status")?)?,
            image: row.get(translator.field("image")?)?,
            created_at: row.get(translator.field("created_at")?)?,
            updated_at: row.get(translator.field("updated_at")?)?,
            category: None,
            unit: None,
            ware_houses: Vec::new(),
        })
    }
    pub fn related_from_row(
        &mut self,
        row: &duckdb::Row,
        translator: &DbTranslateBox,
    ) -> Result<()> {
        if let None = self.unit {
            let unit_id_field: Result<usize> = translator.field("unit_id");
            if let Ok(unit_id_field) = unit_id_field {
                let unit_id: Result<usize> = row.get(unit_id_field).map_err(Error::from);
                if let Ok(_) = unit_id {
                    if let Ok(rel) = translator.with_rel("unit") {
                        let unit: MeasurementUnit = MeasurementUnit::from_row(row, &rel)?;
                        self.unit = Some(unit);
                    }
                }
            }
        }
        if let None = self.category {
            let category_id_field: Result<usize> = translator.field("category_id");
            if let Ok(category_id_field) = category_id_field {
                let category_id: Result<usize> = row.get(category_id_field).map_err(Error::from);
                if let Ok(_) = category_id {
                    if let Ok(rel) = translator.with_rel("category") {
                        let category: ProductCategory = ProductCategory::from_row(row, &rel)?;
                        self.category = Some(category);
                    }
                }
            }
        }
        Ok(())
    }
    pub fn bulk_warehouses_load<'a>(
        conn: &'a Connection,
        products: &mut Vec<Product>,
    ) -> Result<()> {
        if products.len() < 1 {
            return Ok(());
        }
        // let ids = products
        //     .iter()
        //     .map(|e| Value::BigInt(e.id))
        //     .collect::<Vec<Value>>();
        let ids = products.iter().map(|e| e.id).collect::<Vec<i64>>();
        let mut warehouses: Vec<ProductWarehouse> = Vec::new();
        let mut selection = ProductWarehouse::select();
        //
        //
        // !
        // ! WARN: Fix after `List` is implemented by Duckdb!
        // !
        //
        let unstable_list: String = ids
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let unstable_condition: String = format!("({})", unstable_list);
        selection.filter(
            Internal::Field("product_id".into()),
            Operations::In,
            Internal::Field(unstable_condition.into()),
        );
        // !
        // !END
        // !

        // selection.filter(
        //     Internal::Field("product_id".into()),
        //     Operations::In,
        //     Internal::Value(Value::List(ids).into()),
        // );

        let (stmt, translate) = selection.all(&conn)?;

        let mut rows = stmt.raw_query();
        while let Some(row) = rows.next()? {
            let warehouse = ProductWarehouse::from_row(row, &translate)?;

            if !(warehouses.iter().filter(|&e| e.id == warehouse.id).count() > 0) {
                warehouses.push(warehouse);
            }
        }

        for product in products.iter_mut() {
            let mut product_warehouses: Vec<ProductWarehouse> = warehouses
                .iter()
                .filter(|&e| e.product_id == product.id)
                .cloned()
                .collect();
            if product_warehouses.len() > 0 {
                product.ware_houses.append(&mut product_warehouses);
            }
        }

        Ok(())
    }
    pub fn load_warehouses<'a>(&mut self, conn: &'a Connection) -> Result<()> {
        let mut warehouses: Vec<ProductWarehouse> = Vec::new();
        let mut selection = ProductWarehouse::select();
        selection.filter(
            Internal::Field("product_id".into()),
            Operations::EqualTo,
            Internal::Value(self.id.into()),
        );

        let (stmt, translate) = selection.all(&conn)?;

        let mut rows = stmt.raw_query();
        while let Some(row) = rows.next()? {
            let warehouse = ProductWarehouse::from_row(row, &translate)?;
            if !(warehouses.iter().filter(|&e| e.id == warehouse.id).count() > 0) {
                warehouses.push(warehouse);
            }
        }
        self.ware_houses.append(&mut warehouses);
        Ok(())
    }
    pub fn save<'a>(&mut self, conn: &'a Connection) -> Result<()> {
        let mut params: Vec<Rc<dyn ToSql>> = Vec::new();
        let query;
        let now = Utc::now();

        if self.id > 0 {
            self.updated_at = Some(now.naive_utc());
            query = format!(
                "UPDATE {} SET title=? , code=? , base_price=? , inventory=? , initial_inventory=? , status=? , image=? , category_id = ? , unit_id = ? , updated_at=? WHERE id = ?",
                Self::TABLE_NAME
            );
            params.push(Rc::new(self.title.clone()));
            params.push(Rc::new(self.code.clone()));
            params.push(Rc::new(self.base_price.clone()));
            params.push(Rc::new(self.inventory.clone()));
            params.push(Rc::new(self.initial_inventory.clone()));
            params.push(Rc::new(self.status.clone()));
            params.push(Rc::new(self.image.clone()));
            if let Some(category) = &self.category {
                params.push(Rc::new(category.id.clone()));
            } else {
                params.push(Rc::new(None::<i64>));
            }
            if let Some(unit) = &self.unit {
                params.push(Rc::new(unit.id.clone()));
            } else {
                params.push(Rc::new(None::<i64>));
            }
            params.push(Rc::new(self.updated_at.clone()));
            params.push(Rc::new(self.id.clone()));
        } else {
            self.created_at = Some(now.naive_utc());
            self.updated_at = Some(now.naive_utc());
            params.push(Rc::new(self.title.clone()));
            params.push(Rc::new(self.code.clone()));
            params.push(Rc::new(self.base_price.clone()));
            params.push(Rc::new(self.inventory.clone()));
            params.push(Rc::new(self.initial_inventory.clone()));
            params.push(Rc::new(self.status.clone()));
            params.push(Rc::new(self.image.clone()));
            if let Some(category) = &self.category {
                params.push(Rc::new(category.id.clone()));
            } else {
                params.push(Rc::new(None::<i64>));
            }
            if let Some(unit) = &self.unit {
                params.push(Rc::new(unit.id.clone()));
            } else {
                params.push(Rc::new(None::<i64>));
            }
            params.push(Rc::new(self.created_at.clone()));
            params.push(Rc::new(self.updated_at.clone()));
            query = format!(
                "INSERT INTO {} (title,code,base_price,inventory,initial_inventory,status,image,category_id,unit_id,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?) RETURNING id",
                Self::TABLE_NAME
            );
        }
        if self.id > 0 {
            // Update in the database
            let mut stmt = conn.prepare(&query)?;
            let changes = stmt.execute(params_from_iter(params.into_iter()))?;
            if changes <= 0 {
                todo!("Add warning?")
            }
        } else {
            let mut stmt = conn.prepare(&query)?;
            stmt.execute(params_from_iter(params.into_iter()))?;

            return match stmt.raw_query().next()? {
                Some(row) => {
                    let id = row.get(stmt.column_index("id")?)?;
                    self.id = id;
                    return Ok(());
                }
                None => Err(custom_error("failed to insert at 'location'!")),
            };
        }
        Ok(())
    }
    // pub fn add_warehouse(&self, m: HashMap<String, ValueRef>) {
    //     // Check if the row has a product
    //     // First load all data into a map
    //     let warehouse_map: HashMap<String, ValueRef> =
    //         flatten_with_prefix(ProductWarehouse::MODEL_QUERY_PREFIX.to_string(), m);

    //     let mut warehouse_id: Option<String> = warehouse_map.get("id");
    //     let mut the_warehouse: Option<ProductWarehouse> = None;
    //     if let Some(warehouse_id) = warehouse_id {
    //         let old_warehouses: Vec<ProductWarehouse> = self
    //             .ware_houses
    //             .iter()
    //             .filter(|&each| each.id == warehouse_id);
    //         if old_warehouses.iter().count() > 0 {
    //             the_warehouse = old_warehouses.get(0);
    //         }
    //     }
    //     if let None = the_warehouse {
    //         if let Ok(tmp_warehouse) = ProductWarehouse::from_map(warehouse_map, false) {
    //             the_warehouse = Some(tmp_warehouse);
    //         } else {
    //             return; // Failed to set warehouse
    //         }
    //         self.ware_houses.append(the_warehouse.unwrap());
    //     }
    //     the_warehouse?.set_warehouse(warehouse_map);
    // }
    // pub fn set_unit<T>(&self, m: HashMap<String, T>) {
    //     if Some(_) = self.unit {
    //         return;
    //     }
    //     let unit = MeasurementUnit::from_map(m);
    //     if let Ok(unit) = unit {
    //         self.unit = Some(unit);
    //     } else {
    //         return; // Failed to set unit
    //     }
    // }
    // pub fn set_category<T>(&self, m: HashMap<String, T>) {
    //     if Some(_) = self.category {
    //         return;
    //     }
    //     let category = ProductCategory::from_map(m);
    //     if let Ok(category) = category {
    //         self.category = Some(category);
    //     } else {
    //         return; // Failed to set category
    //     }
    // }
}
