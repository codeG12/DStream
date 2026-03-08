use sqlx::PgPool;

use super::models::{CreateStream, StreamRow};

pub async fn create(pool: &PgPool, input: CreateStream) -> sqlx::Result<StreamRow> {
    sqlx::query_as::<_, StreamRow>(
        r#"
        INSERT INTO streams (stream_name, source_connector_id, target_connector_id)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&input.stream_name)
    .bind(input.source_connector_id)
    .bind(input.target_connector_id)
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, stream_id: i32) -> sqlx::Result<Option<StreamRow>> {
    sqlx::query_as::<_, StreamRow>("SELECT * FROM streams WHERE stream_id = $1")
        .bind(stream_id)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &PgPool) -> sqlx::Result<Vec<StreamRow>> {
    sqlx::query_as::<_, StreamRow>("SELECT * FROM streams ORDER BY stream_id")
        .fetch_all(pool)
        .await
}

pub async fn list_by_connector(pool: &PgPool, connector_id: i32) -> sqlx::Result<Vec<StreamRow>> {
    sqlx::query_as::<_, StreamRow>(
        r#"
        SELECT * FROM streams
        WHERE source_connector_id = $1 OR target_connector_id = $1
        ORDER BY stream_id
        "#,
    )
    .bind(connector_id)
    .fetch_optional(pool)
    .await
    .map(|opt| opt.into_iter().collect())
}

pub async fn update_sync_status(
    pool: &PgPool,
    stream_id: i32,
    status: &str,
) -> sqlx::Result<StreamRow> {
    sqlx::query_as::<_, StreamRow>(
        r#"
        UPDATE streams
        SET last_sync_status = $2, last_sync_at = NOW(), updated_at = NOW()
        WHERE stream_id = $1
        RETURNING *
        "#,
    )
    .bind(stream_id)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn set_active(
    pool: &PgPool,
    stream_id: i32,
    active: bool,
) -> sqlx::Result<StreamRow> {
    sqlx::query_as::<_, StreamRow>(
        r#"
        UPDATE streams
        SET is_active = $2, updated_at = NOW()
        WHERE stream_id = $1
        RETURNING *
        "#,
    )
    .bind(stream_id)
    .bind(active)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, stream_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM streams WHERE stream_id = $1")
        .bind(stream_id)
        .execute(pool)
        .await?;
    Ok(())
}
