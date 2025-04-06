use crate::register_migration;

// Register all migrations for location service
pub fn register_migrations() {
    register_migration!(
        "location",
        "Create location table",
        1,
        "CREATE TABLE IF NOT EXISTS location (data JSON)",
        "DROP TABLE IF EXISTS location"
    );
}
