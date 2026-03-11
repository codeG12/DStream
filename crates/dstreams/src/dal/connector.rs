use serde_json::Value;
use sqlx::PgPool;

use super::models::{ConnectorRow, CreateConnector};

pub async fn create(pool: &PgPool, input: CreateConnector) -> sqlx::Result<ConnectorRow> {
    sqlx::query_as::<_, ConnectorRow>(
        r#"
        INSERT INTO connectors (connector_name, connector_type, config)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&input.connector_name)
    .bind(&input.connector_type)
    .bind(&input.config)
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, connector_id: i32) -> sqlx::Result<Option<ConnectorRow>> {
    sqlx::query_as::<_, ConnectorRow>("SELECT * FROM connectors WHERE connector_id = $1")
        .bind(connector_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_by_name(pool: &PgPool, name: &str) -> sqlx::Result<Option<ConnectorRow>> {
    sqlx::query_as::<_, ConnectorRow>("SELECT * FROM connectors WHERE connector_name = $1")
        .bind(name)
        .fetch_optional(pool)
        .await
}

pub async fn list(pool: &PgPool) -> sqlx::Result<Vec<ConnectorRow>> {
    sqlx::query_as::<_, ConnectorRow>("SELECT * FROM connectors ORDER BY connector_id")
        .fetch_all(pool)
        .await
}

pub async fn update_config(
    pool: &PgPool,
    connector_id: i32,
    config: Value,
) -> sqlx::Result<ConnectorRow> {
    sqlx::query_as::<_, ConnectorRow>(
        r#"
        UPDATE connectors
        SET config = $2, updated_at = NOW()
        WHERE connector_id = $1
        RETURNING *
        "#,
    )
    .bind(connector_id)
    .bind(&config)
    .fetch_one(pool)
    .await
}

pub async fn set_active(
    pool: &PgPool,
    connector_id: i32,
    active: bool,
) -> sqlx::Result<ConnectorRow> {
    sqlx::query_as::<_, ConnectorRow>(
        r#"
        UPDATE connectors
        SET is_active = $2, updated_at = NOW()
        WHERE connector_id = $1
        RETURNING *
        "#,
    )
    .bind(connector_id)
    .bind(active)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &PgPool, connector_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM connectors WHERE connector_id = $1")
        .bind(connector_id)
        .execute(pool)
        .await?;
    Ok(())
}
