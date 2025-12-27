use sqlx::{PgPool, Result};
use serde_json::Value;

pub async fn create_connector(
    pool: &PgPool,
    connector_name: &str,
    connector_type: &str,
    config: Value,
) -> Result<i32> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO connectors (connector_name, connector_type, config)
        VALUES ($1, $2, $3)
        RETURNING connector_id
        "#,
        connector_name,
        connector_type,
        config
    )
        .fetch_one(pool)
        .await?;

    Ok(rec.connector_id)
}
