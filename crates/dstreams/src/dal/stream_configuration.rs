use sqlx::PgPool;

use super::models::{CreateStreamConfiguration, StreamConfigurationRow};

pub async fn create(
    pool: &PgPool,
    input: CreateStreamConfiguration,
) -> sqlx::Result<StreamConfigurationRow> {
    sqlx::query_as::<_, StreamConfigurationRow>(
        r#"
        INSERT INTO stream_configuration (
            stream_id, catalog_item_id, is_selected, replication_method, replication_key
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(input.stream_id)
    .bind(input.catalog_item_id)
    .bind(input.is_selected)
    .bind(&input.replication_method)
    .bind(&input.replication_key)
    .fetch_one(pool)
    .await
}

pub async fn list_by_stream(
    pool: &PgPool,
    stream_id: i32,
) -> sqlx::Result<Vec<StreamConfigurationRow>> {
    sqlx::query_as::<_, StreamConfigurationRow>(
        "SELECT * FROM stream_configuration WHERE stream_id = $1 ORDER BY id",
    )
    .bind(stream_id)
    .fetch_all(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM stream_configuration WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
