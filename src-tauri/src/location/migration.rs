use crate::register_migration;

// Register all migrations for location service
pub async fn register_migrations() {
    // Migration 1: Create location sequence
    register_migration!(
        "location",
        "Create location sequence",
        1,
        r#"
        CREATE SEQUENCE location_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS location_id_seq"
    )
    .await;

    // Migration 2: Create location table
    register_migration!(
        "location",
        "Create location table",
        2,
        r#"
        CREATE TABLE location (
            id INTEGER PRIMARY KEY DEFAULT nextval('location_id_seq'),
            name TEXT NOT NULL,
            created_at TIMESTAMP,
            updated_at TIMESTAMP
        )
        "#,
        "DROP TABLE IF EXISTS location",
        "location.1"
    )
    .await;
}
