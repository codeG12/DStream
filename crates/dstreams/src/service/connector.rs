use anyhow::{Context, Result};
use serde_json::Value;

use crate::core::config::{TapConfig, TargetConfig};
use crate::dal::connector;
use crate::dal::models::{ConnectorRow, CreateConnector};
use crate::dal::DbPool;

/// Register a tap connector — validates config and persists to DB.
pub async fn register_tap(pool: &DbPool, config: &TapConfig) -> Result<ConnectorRow> {
    config.validate().context("Invalid tap configuration")?;

    let config_json = serde_json::to_value(config).context("Failed to serialize tap config")?;

    let input = CreateConnector {
        connector_name: config.name.clone(),
        connector_type: "tap".to_string(),
        config: config_json,
    };

    let row = connector::create(pool, input)
        .await
        .context("Failed to insert tap connector")?;

    tracing::info!(
        connector_id = row.connector_id,
        name = %row.connector_name,
        "Registered tap connector"
    );

    Ok(row)
}

/// Register a target connector — validates config and persists to DB.
pub async fn register_target(pool: &DbPool, config: &TargetConfig) -> Result<ConnectorRow> {
    config.validate().context("Invalid target configuration")?;

    let config_json = serde_json::to_value(config).context("Failed to serialize target config")?;

    let input = CreateConnector {
        connector_name: config.name.clone(),
        connector_type: "target".to_string(),
        config: config_json,
    };

    let row = connector::create(pool, input)
        .await
        .context("Failed to insert target connector")?;

    tracing::info!(
        connector_id = row.connector_id,
        name = %row.connector_name,
        "Registered target connector"
    );

    Ok(row)
}

/// Fetch a connector by ID.
pub async fn get_connector(pool: &DbPool, connector_id: i32) -> Result<ConnectorRow> {
    connector::get_by_id(pool, connector_id)
        .await
        .context("Database query failed")?
        .context("Connector not found")
}

/// Fetch a connector by name.
pub async fn get_connector_by_name(pool: &DbPool, name: &str) -> Result<ConnectorRow> {
    connector::get_by_name(pool, name)
        .await
        .context("Database query failed")?
        .context("Connector not found")
}

/// List all connectors.
pub async fn list_connectors(pool: &DbPool) -> Result<Vec<ConnectorRow>> {
    connector::list(pool)
        .await
        .context("Failed to list connectors")
}

/// Update a connector's config.
pub async fn update_connector_config(
    pool: &DbPool,
    connector_id: i32,
    config: Value,
) -> Result<ConnectorRow> {
    connector::update_config(pool, connector_id, config)
        .await
        .context("Failed to update connector config")
}

/// Deactivate a connector (soft-delete).
pub async fn deactivate_connector(pool: &DbPool, connector_id: i32) -> Result<ConnectorRow> {
    let row = connector::set_active(pool, connector_id, false)
        .await
        .context("Failed to deactivate connector")?;

    tracing::info!(connector_id, "Connector deactivated");

    Ok(row)
}
