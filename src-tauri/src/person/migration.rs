use crate::register_migration;

// Register all migrations for person service
pub async fn register_migrations() {
    register_migration!(
        "person",
        "Create person table",
        1,
        "CREATE TABLE IF NOT EXISTS person (data JSON)",
        "DROP TABLE IF EXISTS person"
    )
    .await;
}
