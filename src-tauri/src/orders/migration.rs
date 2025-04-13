use crate::register_migration;

// Register all migrations for orders service
pub async fn register_migrations() {
    // Migration 3: Create order sequence
    register_migration!(
        "orders",
        "Create order sequence",
        1,
        r#"
        CREATE SEQUENCE order_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS order_id_seq"
    )
    .await;

    // Migration 4: Create order table
    register_migration!(
        "orders",
        "Create order table",
        2,
        r#"
        CREATE TABLE order_table (
            id INTEGER PRIMARY KEY DEFAULT nextval('order_id_seq'),
            order_type TEXT NOT NULL,
            description TEXT NOT NULL,
            citation_number TEXT NOT NULL,
            document_date TEXT NOT NULL,
            document_number TEXT NOT NULL,
            status TEXT NOT NULL,
            delivery_type TEXT,
            delivery_id INTEGER,
            recipient_type TEXT,
            recipient_id INTEGER,
            manager_id INTEGER,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            FOREIGN KEY (manager_id) REFERENCES person(id)
        )
        "#,
        "DROP TABLE IF EXISTS order_table"
    )
    .await;

    // Migration 1: Create order_role sequence
    register_migration!(
        "orders",
        "Create order_role sequence",
        3,
        r#"
        CREATE SEQUENCE order_role_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS order_role_id_seq"
    )
    .await;

    // Migration 2: Create order_role table
    register_migration!(
        "orders",
        "Create order_role table",
        4,
        r#"
        CREATE TABLE order_role (
            id INTEGER PRIMARY KEY DEFAULT nextval('order_role_id_seq'),
            name TEXT NOT NULL,
            person_id INTEGER,
            order_id INTEGER,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            FOREIGN KEY (person_id) REFERENCES person(id),
            FOREIGN KEY (order_id) REFERENCES order_table(id)
        )
        "#,
        "DROP TABLE IF EXISTS order_role"
    )
    .await;

    // Migration 5: Create order_product sequence
    register_migration!(
        "orders",
        "Create order_product sequence",
        5,
        r#"
        CREATE SEQUENCE order_product_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS order_product_id_seq"
    )
    .await;

    // Migration 6: Create order_product table
    register_migration!(
        "orders",
        "Create order_product table",
        6,
        r#"
        CREATE TABLE order_product (
            id INTEGER PRIMARY KEY DEFAULT nextval('order_product_id_seq'),
            order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            FOREIGN KEY (order_id) REFERENCES order_table(id),
            FOREIGN KEY (product_id) REFERENCES product(id)
        )
        "#,
        "DROP TABLE IF EXISTS order_product"
    )
    .await;

    // Migration 9: Create order_user sequence
    register_migration!(
        "orders",
        "Create order_user sequence",
        7,
        r#"
        CREATE SEQUENCE order_user_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS order_user_id_seq"
    )
    .await;

    // Migration 10: Create order_user table (many-to-many relationship)
    register_migration!(
        "orders",
        "Create order_user table",
        8,
        r#"
        CREATE TABLE order_user (
            id INTEGER PRIMARY KEY DEFAULT nextval('order_user_id_seq'),
            order_id INTEGER NOT NULL,
            location_id INTEGER NOT NULL,
            FOREIGN KEY (order_id) REFERENCES order_table(id),
            FOREIGN KEY (location_id) REFERENCES location(id)
        )
        "#,
        "DROP TABLE IF EXISTS order_user"
    )
    .await;
}
