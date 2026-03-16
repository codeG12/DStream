use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::core::errors::{ConfigError, DStreamError, Result};

pub async fn client() -> Result<PgPool> {
    let database_url = env::var("POSTGRES_DB")
        .map_err(|_| DStreamError::Config(ConfigError::MissingField("POSTGRES_DB environment variable not set".to_string())))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| DStreamError::Custom(format!("Database connection failed: {}", e)))?;

    Ok(pool)
}
