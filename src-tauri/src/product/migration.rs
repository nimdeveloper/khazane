use crate::register_migration;

// Register all migrations for product service
pub async fn register_migrations() {
    // Migration 1: Create category sequence
    register_migration!(
        "product",
        "Create category sequence",
        1,
        r#"
        CREATE SEQUENCE category_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS category_id_seq"
    )
    .await;

    // Migration 2: Create category table
    register_migration!(
        "product",
        "Create category table",
        2,
        r#"
        CREATE TABLE category (
            id INTEGER PRIMARY KEY DEFAULT nextval('category_id_seq'),
            label TEXT NOT NULL,
            created_at TIMESTAMP,
            updated_at TIMESTAMP
        )
        "#,
        "DROP TABLE IF EXISTS category",
        "product.1"
    )
    .await;

    // Migration 3: Create measure_unit sequence
    register_migration!(
        "product",
        "Create measure_unit sequence",
        3,
        r#"
        CREATE SEQUENCE measure_unit_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS measure_unit_id_seq"
    )
    .await;

    // Migration 4: Create measure_unit table
    register_migration!(
        "product",
        "Create measure_unit table",
        4,
        r#"
        CREATE TABLE measure_unit (
            id INTEGER PRIMARY KEY DEFAULT nextval('measure_unit_id_seq'),
            title TEXT NOT NULL,
            created_at TIMESTAMP,
            updated_at TIMESTAMP
        )
        "#,
        "DROP TABLE IF EXISTS measure_unit",
        "product.3"
    )
    .await;

    // Migration 5: Create product sequence
    register_migration!(
        "product",
        "Create product sequence",
        5,
        r#"
        CREATE SEQUENCE product_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS product_id_seq"
    )
    .await;

    // Migration 6: Create product table
    register_migration!(
        "product",
        "Create product table",
        6,
        r#"
        CREATE TABLE product (
            id INTEGER PRIMARY KEY DEFAULT nextval('product_id_seq'),
            title TEXT NOT NULL,
            code TEXT NOT NULL,
            unit_id INTEGER,
            base_price INTEGER NOT NULL,
            inventory INTEGER NOT NULL,
            initial_inventory INTEGER NOT NULL,
            status TEXT NOT NULL,
            image TEXT,
            category_id INTEGER,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            FOREIGN KEY (unit_id) REFERENCES measure_unit(id),
            FOREIGN KEY (category_id) REFERENCES category(id)
        )
        "#,
        "DROP TABLE IF EXISTS product",
        "product.5",
        "product.2",
        "product.4"
    )
    .await;

    // Migration 7: Create product_warehouse sequence
    register_migration!(
        "product",
        "Create product_warehouse sequence",
        7,
        r#"
        CREATE SEQUENCE product_warehouse_id_seq START WITH 1 INCREMENT BY 1;
        "#,
        "DROP SEQUENCE IF EXISTS product_warehouse_id_seq"
    )
    .await;

    // Migration 8: Create product_warehouse table (many-to-many relationship)
    register_migration!(
        "product",
        "Create product_warehouse table",
        8,
        r#"
        CREATE TABLE product_warehouse (
            id INTEGER PRIMARY KEY DEFAULT nextval('product_warehouse_id_seq'),
            product_id INTEGER NOT NULL,
            warehouse_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            created_at TIMESTAMP,
            updated_at TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES product(id),
            FOREIGN KEY (warehouse_id) REFERENCES warehouse(id)
        )
        "#,
        "DROP TABLE IF EXISTS product_warehouse",
        "product.6",
        "product.7",
        "warehouse.2"
    )
    .await;
}
