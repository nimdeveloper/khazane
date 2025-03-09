use surrealdb::{engine::local::Db, Surreal};

#[derive(Default)]
pub struct AppData {
    pub db: Option<Surreal<Db>>,
}
