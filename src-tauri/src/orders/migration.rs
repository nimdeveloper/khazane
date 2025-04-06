use crate::register_migration;

// Register all migrations for orders service
pub fn register_migrations() {
    register_migration!(
        "orders",
        "Create orders table",
        1,
        "CREATE TABLE IF NOT EXISTS order_table (data JSON)",
        "DROP TABLE IF EXISTS order_table"
    );
}
