use sqlx::PgPool;

use super::models::{StateRow, UpsertState};

pub async fn upsert(pool: &PgPool, input: UpsertState) -> sqlx::Result<StateRow> {
    sqlx::query_as::<_, StateRow>(
        r#"
        INSERT INTO state (
            stream_id, table_name, bookmark_column,
            bookmark_value, bookmark_type, records_synced, last_sync_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        ON CONFLICT (stream_id, table_name)
        DO UPDATE SET
            bookmark_column = EXCLUDED.bookmark_column,
            bookmark_value = EXCLUDED.bookmark_value,
            bookmark_type = EXCLUDED.bookmark_type,
            records_synced = EXCLUDED.records_synced,
            last_sync_at = NOW(),
            updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(input.stream_id)
    .bind(&input.table_name)
    .bind(&input.bookmark_column)
    .bind(&input.bookmark_value)
    .bind(&input.bookmark_type)
    .bind(input.records_synced)
    .fetch_one(pool)
    .await
}

pub async fn get(
    pool: &PgPool,
    stream_id: i32,
    table_name: &str,
) -> sqlx::Result<Option<StateRow>> {
    sqlx::query_as::<_, StateRow>("SELECT * FROM state WHERE stream_id = $1 AND table_name = $2")
        .bind(stream_id)
        .bind(table_name)
        .fetch_optional(pool)
        .await
}

pub async fn list_by_stream(pool: &PgPool, stream_id: i32) -> sqlx::Result<Vec<StateRow>> {
    sqlx::query_as::<_, StateRow>("SELECT * FROM state WHERE stream_id = $1 ORDER BY table_name")
        .bind(stream_id)
        .fetch_all(pool)
        .await
}

pub async fn delete(pool: &PgPool, stream_id: i32, table_name: &str) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM state WHERE stream_id = $1 AND table_name = $2")
        .bind(stream_id)
        .bind(table_name)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn increment_records(
    pool: &PgPool,
    stream_id: i32,
    table_name: &str,
    count: i64,
) -> sqlx::Result<StateRow> {
    sqlx::query_as::<_, StateRow>(
        r#"
        UPDATE state
        SET records_synced = COALESCE(records_synced, 0) + $3,
            last_sync_at = NOW(),
            updated_at = NOW()
        WHERE stream_id = $1 AND table_name = $2
        RETURNING *
        "#,
    )
    .bind(stream_id)
    .bind(table_name)
    .bind(count)
    .fetch_one(pool)
    .await
}
