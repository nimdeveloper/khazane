use crate::register_migration;

// Register all migrations for person service
pub async fn register_migrations() {
    // Migration 1: Create person sequence
    register_migration!(
        "person",
        "Create person sequence",
        1,
        r#"
        CREATE SEQUENCE person_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS person_id_seq"
    )
    .await;

    // Migration 2: Create person table
    register_migration!(
        "person",
        "Create person table",
        2,
        r#"
        CREATE TABLE person (
            id INTEGER PRIMARY KEY DEFAULT nextval('person_id_seq'),
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            national_code TEXT,
            phone TEXT,
            email TEXT,
            address TEXT,
            created_at TIMESTAMP,
            updated_at TIMESTAMP
        )
        "#,
        "DROP TABLE IF EXISTS person"
    )
    .await;
}
