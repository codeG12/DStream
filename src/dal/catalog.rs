use sqlx::PgPool;

use super::models::{CatalogRow, CreateCatalogEntry};

pub async fn create(pool: &PgPool, input: CreateCatalogEntry) -> sqlx::Result<CatalogRow> {
    sqlx::query_as::<_, CatalogRow>(
        r#"
        INSERT INTO catalog (
            connector_id, table_name, schema_name, table_schema,
            key_properties, replication_method, replication_key, is_selected
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(input.connector_id)
    .bind(&input.table_name)
    .bind(&input.schema_name)
    .bind(&input.table_schema)
    .bind(&input.key_properties)
    .bind(&input.replication_method)
    .bind(&input.replication_key)
    .bind(input.is_selected)
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, catalog_id: i32) -> sqlx::Result<Option<CatalogRow>> {
    sqlx::query_as::<_, CatalogRow>("SELECT * FROM catalog WHERE catalog_id = $1")
        .bind(catalog_id)
        .fetch_optional(pool)
        .await
}

pub async fn list_by_connector(
    pool: &PgPool,
    connector_id: i32,
) -> sqlx::Result<Vec<CatalogRow>> {
    sqlx::query_as::<_, CatalogRow>(
        "SELECT * FROM catalog WHERE connector_id = $1 ORDER BY catalog_id",
    )
    .bind(connector_id)
    .fetch_all(pool)
    .await
}

pub async fn set_selected(
    pool: &PgPool,
    catalog_id: i32,
    selected: bool,
) -> sqlx::Result<CatalogRow> {
    sqlx::query_as::<_, CatalogRow>(
        r#"
        UPDATE catalog
        SET is_selected = $2, updated_at = NOW()
        WHERE catalog_id = $1
        RETURNING *
        "#,
    )
    .bind(catalog_id)
    .bind(selected)
    .fetch_one(pool)
    .await
}

pub async fn upsert(pool: &PgPool, input: CreateCatalogEntry) -> sqlx::Result<CatalogRow> {
    sqlx::query_as::<_, CatalogRow>(
        r#"
        INSERT INTO catalog (
            connector_id, table_name, schema_name, table_schema,
            key_properties, replication_method, replication_key, is_selected
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (connector_id, table_name, schema_name)
        DO UPDATE SET
            table_schema = EXCLUDED.table_schema,
            key_properties = EXCLUDED.key_properties,
            replication_method = EXCLUDED.replication_method,
            replication_key = EXCLUDED.replication_key,
            is_selected = EXCLUDED.is_selected,
            updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(input.connector_id)
    .bind(&input.table_name)
    .bind(&input.schema_name)
    .bind(&input.table_schema)
    .bind(&input.key_properties)
    .bind(&input.replication_method)
    .bind(&input.replication_key)
    .bind(input.is_selected)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, catalog_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM catalog WHERE catalog_id = $1")
        .bind(catalog_id)
        .execute(pool)
        .await?;
    Ok(())
}
