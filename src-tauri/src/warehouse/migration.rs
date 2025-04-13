use crate::register_migration;

// Register all migrations for warehouse service
pub async fn register_migrations() {
    // Migration 1: Create warehouse sequence
    register_migration!(
        "warehouse",
        "Create warehouse sequence",
        1,
        r#"
        CREATE SEQUENCE warehouse_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS warehouse_id_seq"
    )
    .await;

    // Migration 2: Create warehouse table
    register_migration!(
        "warehouse",
        "Create warehouse table",
        2,
        r#"
        CREATE TABLE warehouse (
            id INTEGER PRIMARY KEY DEFAULT nextval('warehouse_id_seq'),
            name TEXT NOT NULL,
            shorthand TEXT NOT NULL,
            color_key TEXT,
            color_code TEXT,
            created_at TIMESTAMP,
            updated_at TIMESTAMP
        )
        "#,
        "DROP TABLE IF EXISTS warehouse"
    )
    .await;
}
