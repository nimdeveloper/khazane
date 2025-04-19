#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::database::value_ref_to_type;
use crate::core::error::{custom_error, Error, Result};
use crate::core::selector::{
    Condition, DbTranslateBox, Internal, Join, JoinMethod, Operations, Selector,
};
use crate::core::{helpers::flatten_with_prefix, repository::Model};
use crate::warehouse::model::Warehouse;
use chrono::{NaiveDateTime, Utc};
use duckdb::types::ValueRef;
use duckdb::{params_from_iter, Connection, Statement, ToSql};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductWarehouse {
    pub id: String,
    pub quantity: i64,
    pub warehouse: Option<Warehouse>,
}
impl ProductWarehouse {
    // pub fn as_join() {
    //     Join::to(Self::TABLE_NAME, JoinMethod::LeftJoinJoin)
    //         .on("id", "warehouse_id")
    //         .with(Warehouse::TABLE_NAME)
    //         .as("warehouse")
    // }
    pub fn get_columns() -> [String; 3] {
        [
            "id".to_string(),
            "quantity".to_string(),
            "warehouse_id".to_string(),
        ]
    }
    pub fn from_map(m: HashMap<String, ValueRef>, load_relations: bool) -> Result<Self> {
        // if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
        //     return Err(custom_error(
        //         "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
        //     ));
        // }
        let id: String = value_ref_to_type(m.get("id").unwrap())?;
        let quantity: i64 = value_ref_to_type(m.get("quantity").unwrap())?;

        let res = ProductWarehouse {
            id,
            quantity,
            warehouse: None,
        };
        if load_relations {
            res.set_warehouse(m);
        }
        Ok(res)
    }
    pub fn set_warehouse(&self, m: HashMap<String, ValueRef>) {
        // if let Some(_) = self.warehouse {
        //     return;
        // }
        // if !m.contains_key(format!("{}id", (Warehouse::MODEL_QUERY_PREFIX)).as_str()) {
        //     return; // No warehouse_id in the map
        // }
        // let warehouse = Warehouse::from_map(flatten_with_prefix(
        //     Warehouse::MODEL_QUERY_PREFIX.to_string(),
        //     m,
        // ));
        // if let Ok(warehouse) = warehouse {
        //     self.warehouse = Some(warehouse);
        // } else {
        //     return; // Failed to set warehouse
        // }
        return;
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

#[derive(Debug, Serialize, Deserialize)]
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
                    Internal::Field("id".into()),
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
                    Internal::Field("id".into()),
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
