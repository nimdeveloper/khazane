use crate::register_migration;

// Register all migrations for warehouse service
pub fn register_migrations() {
    register_migration!(
        "warehouse",
        "Create warehouse table",
        1,
        "CREATE TABLE IF NOT EXISTS warehouse (data JSON)",
        "DROP TABLE IF EXISTS warehouse"
    );
}
