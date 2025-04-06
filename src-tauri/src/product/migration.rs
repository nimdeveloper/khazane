use crate::register_migration;

// Register all migrations for product service
pub async fn register_migrations() {
    register_migration!(
        "product",
        "Create product table",
        1,
        "CREATE TABLE IF NOT EXISTS product (data JSON)",
        "DROP TABLE IF EXISTS product"
    )
    .await;

    register_migration!(
        "product",
        "Create category table",
        2,
        "CREATE TABLE IF NOT EXISTS category (data JSON)",
        "DROP TABLE IF EXISTS category"
    )
    .await;

    register_migration!(
        "product",
        "Create measure unit table",
        3,
        "CREATE TABLE IF NOT EXISTS measure_unit (data JSON)",
        "DROP TABLE IF EXISTS measure_unit"
    )
    .await;
}
