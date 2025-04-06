use crate::core::repository::Model;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Model for Location {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_table_name() -> &'static str {
        "location"
    }
}
