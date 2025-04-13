use std::collections::HashMap;

use crate::core::database::value_ref_to_type;
use crate::core::error::{custom_error, Error};
use crate::core::{helpers::flatten_with_prefix, repository::Model};
use crate::warehouse::model::Warehouse;
use duckdb::types::ValueRef;
use duckdb::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductCategory {
    pub id: String,
    pub label: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl ProductCategory {
    const MODEL_QUERY_PREFIX: &'static str = "category_";
    pub fn get_columns() -> [&'static str; 4] {
        ["id", "label", "created_at", "updated_at"]
    }

    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Self {
        ProductCategory {
            id: row.get(stmt.column_index("id")?)?,
            label: row.get(stmt.column_index("label")?)?,
            created_at: row.get(stmt.column_index("created_at")?)?,
            updated_at: row.get(stmt.column_index("updated_at")?)?,
        }
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self, Error> {
        if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
            return Err(custom_error(
                "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
            ));
        }
        let id: Result<String> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let label: Result<String> = value_ref_to_type(m.get("label").unwrap()).map_err(Error::from);
        if let Err(e) = label {
            return Err(e);
        }
        let label = label.unwrap();

        let created_at: Result<chrono::NaiveDateTime> =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from);
        if let Err(e) = created_at {
            return Err(e);
        }
        let created_at = created_at.unwrap();

        let updated_at: Result<chrono::NaiveDateTime> =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from);
        if let Err(e) = updated_at {
            return Err(e);
        }
        let updated_at = updated_at.unwrap();

        Ok(ProductCategory {
            id,
            label,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
}
impl Model for ProductCategory {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "category"
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductWarehouse {
    pub id: String,
    pub quantity: i64,
    pub warehouse: Option<Warehouse>,
}
impl ProductWarehouse {
    const MODEL_QUERY_PREFIX: &'static str = "warehouse_";
    pub fn get_columns() -> [&'static str; 2] {
        ["id", "quantity"]
    }
    pub fn from_map(m: HashMap<String, ValueRef>, load_relations: bool) -> Result<Self, Error> {
        if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
            return Err(custom_error(
                "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
            ));
        }
        let id: Result<String> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let quantity: Result<i64> =
            value_ref_to_type(m.get("quantity").unwrap()).map_err(Error::from);
        if let Err(e) = quantity {
            return Err(e);
        }
        let quantity = quantity.unwrap();

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
        if Some(_) = self.warehouse {
            return;
        }
        if !m.contains_key(format!("{}id", (Warehouse::MODEL_QUERY_PREFIX)).as_str()) {
            return; // No warehouse_id in the map
        }
        let warehouse = Warehouse::from_map(flatten_with_prefix(
            Warehouse::MODEL_QUERY_PREFIX.to_string(),
            m,
        ));
        if let Ok(warehouse) = warehouse {
            self.warehouse = Some(warehouse);
        } else {
            return; // Failed to set warehouse
        }
        return;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeasurementUnit {
    pub id: String,
    pub title: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl MeasurementUnit {
    const MODEL_QUERY_PREFIX: &'static str = "unit_";
    pub fn get_columns() -> [&'static str; 4] {
        ["id", "title", "created_at", "updated_at"]
    }

    pub fn from_row(row: &duckdb::Row, stmt: &Statement) -> Self {
        MeasurementUnit {
            id: row.get(stmt.column_index("id")?)?,
            title: row.get(stmt.column_index("title")?)?,
            created_at: row.get(stmt.column_index("created_at")?)?,
            updated_at: row.get(stmt.column_index("updated_at")?)?,
        }
    }
    pub fn from_map(m: HashMap<String, ValueRef>) -> Result<Self, Error> {
        if Self::get_columns().iter().any(|&e| !m.contains_key(e)) {
            return Err(custom_error(
                "Failed to construct MeasurementUnit from HasMap! Some keys missing!",
            ));
        }
        let id: Result<String> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let title: Result<String> = value_ref_to_type(m.get("title").unwrap()).map_err(Error::from);
        if let Err(e) = title {
            return Err(e);
        }
        let title = title.unwrap();

        let created_at: Result<chrono::NaiveDateTime> =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from);
        if let Err(e) = created_at {
            return Err(e);
        }
        let created_at = created_at.unwrap();

        let updated_at: Result<chrono::NaiveDateTime> =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from);
        if let Err(e) = updated_at {
            return Err(e);
        }
        let updated_at = updated_at.unwrap();

        Ok(MeasurementUnit {
            id,
            title,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        })
    }
}
impl Model for MeasurementUnit {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "measure_unit"
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub code: String,
    pub unit: Option<MeasurementUnit>,
    pub base_price: i64,
    pub inventory: i64,
    pub initial_inventory: i64,
    pub status: String,
    pub image: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub ware_houses: Vec<ProductWarehouse>,
    pub category: Option<ProductCategory>,
}
impl Model for Product {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "product"
    }
}

impl Product {
    pub const MODEL_QUERY_PREFIX: &'static str = "product_";
    pub fn from_map(m: HashMap<String, ValueRef>, load_relations: bool) -> Result<Self, Error> {
        let id: Result<String> = value_ref_to_type(m.get("id").unwrap()).map_err(Error::from);
        if let Err(e) = id {
            return Err(e);
        }
        let id = id.unwrap();

        let title: Result<String> = value_ref_to_type(m.get("title").unwrap()).map_err(Error::from);
        if let Err(e) = title {
            return Err(e);
        }
        let title = title.unwrap();

        let code: Result<String> = value_ref_to_type(m.get("code").unwrap()).map_err(Error::from);
        if let Err(e) = code {
            return Err(e);
        }
        let code = code.unwrap();

        let base_price: Result<String> =
            value_ref_to_type(m.get("base_price").unwrap()).map_err(Error::from);
        if let Err(e) = base_price {
            return Err(e);
        }
        let base_price = base_price.unwrap();

        let inventory: Result<String> =
            value_ref_to_type(m.get("inventory").unwrap()).map_err(Error::from);
        if let Err(e) = inventory {
            return Err(e);
        }
        let inventory = inventory.unwrap();

        let initial_inventory: Result<String> =
            value_ref_to_type(m.get("initial_inventory").unwrap()).map_err(Error::from);
        if let Err(e) = initial_inventory {
            return Err(e);
        }
        let initial_inventory = initial_inventory.unwrap();

        let status: Result<String> =
            value_ref_to_type(m.get("status").unwrap()).map_err(Error::from);
        if let Err(e) = status {
            return Err(e);
        }
        let status = status.unwrap();

        let image: Result<String> = value_ref_to_type(m.get("image").unwrap()).map_err(Error::from);
        if let Err(e) = image {
            return Err(e);
        }
        let image = image.unwrap();

        let created_at: Result<String> =
            value_ref_to_type(m.get("created_at").unwrap()).map_err(Error::from);
        if let Err(e) = created_at {
            return Err(e);
        }
        let created_at = created_at.unwrap();

        let updated_at: Result<String> =
            value_ref_to_type(m.get("updated_at").unwrap()).map_err(Error::from);
        if let Err(e) = updated_at {
            return Err(e);
        }
        let updated_at = updated_at.unwrap();

        let mut res = Product {
            id,
            title,
            code,
            unit: None,
            base_price,
            inventory,
            initial_inventory,
            status,
            image,
            created_at,
            updated_at,
            ware_houses: Vec::<ProductWarehouse>::new(),
            category: None,
        };
        if load_relations {
            res.set_unit(m);
            res.set_category(m);
            res.add_warehouse(m);
        }
        Ok(res)
    }
    pub fn add_warehouse(&self, m: HashMap<String, ValueRef>) {
        // Check if the row has a product
        // First load all data into a map
        let warehouse_map: HashMap<String, ValueRef> =
            flatten_with_prefix(ProductWarehouse::MODEL_QUERY_PREFIX.to_string(), m);

        let mut warehouse_id: Option<String> = warehouse_map.get("id");
        let mut the_warehouse: Option<ProductWarehouse> = None;
        if let Some(warehouse_id) = warehouse_id {
            let old_warehouses: Vec<ProductWarehouse> = self
                .ware_houses
                .iter()
                .filter(|&each| each.id == warehouse_id);
            if old_warehouses.iter().count() > 0 {
                the_warehouse = old_warehouses.get(0);
            }
        }
        if let None = the_warehouse {
            if let Ok(tmp_warehouse) = ProductWarehouse::from_map(warehouse_map, false) {
                the_warehouse = Some(tmp_warehouse);
            } else {
                return; // Failed to set warehouse
            }
            self.ware_houses.append(the_warehouse.unwrap());
        }
        the_warehouse?.set_warehouse(warehouse_map);
    }
    pub fn set_unit<T>(&self, m: HashMap<String, T>) {
        if Some(_) = self.unit {
            return;
        }
        let unit = MeasurementUnit::from_map(m);
        if let Ok(unit) = unit {
            self.unit = Some(unit);
        } else {
            return; // Failed to set unit
        }
    }
    pub fn set_category<T>(&self, m: HashMap<String, T>) {
        if Some(_) = self.category {
            return;
        }
        let category = ProductCategory::from_map(m);
        if let Ok(category) = category {
            self.category = Some(category);
        } else {
            return; // Failed to set category
        }
    }
}
