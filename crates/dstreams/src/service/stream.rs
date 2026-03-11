use anyhow::{Context, Result};

use crate::dal::connector;
use crate::dal::models::{CreateStream, StreamRow};
use crate::dal::stream;
use crate::dal::DbPool;

/// Create a new stream pipeline between a source and target connector.
/// Validates that both connectors exist before inserting.
pub async fn create_stream(
    pool: &DbPool,
    stream_name: String,
    source_connector_id: i32,
    target_connector_id: i32,
) -> Result<StreamRow> {
    // Verify source connector exists
    connector::get_by_id(pool, source_connector_id)
        .await
        .context("Database query failed")?
        .context("Source connector not found")?;

    // Verify target connector exists
    connector::get_by_id(pool, target_connector_id)
        .await
        .context("Database query failed")?
        .context("Target connector not found")?;

    let input = CreateStream {
        stream_name,
        source_connector_id,
        target_connector_id,
    };

    let row = stream::create(pool, input)
        .await
        .context("Failed to create stream")?;

    tracing::info!(
        stream_id = row.stream_id,
        name = %row.stream_name,
        "Created stream pipeline"
    );

    Ok(row)
}

/// Fetch a stream by ID.
pub async fn get_stream(pool: &DbPool, stream_id: i32) -> Result<StreamRow> {
    stream::get_by_id(pool, stream_id)
        .await
        .context("Database query failed")?
        .context("Stream not found")
}

/// List all streams.
pub async fn list_streams(pool: &DbPool) -> Result<Vec<StreamRow>> {
    stream::list(pool).await.context("Failed to list streams")
}

/// List all streams associated with a given connector (as source or target).
pub async fn list_streams_by_connector(pool: &DbPool, connector_id: i32) -> Result<Vec<StreamRow>> {
    stream::list_by_connector(pool, connector_id)
        .await
        .context("Failed to list streams by connector")
}

/// Mark a sync as complete with a given status string.
pub async fn mark_sync_complete(pool: &DbPool, stream_id: i32, status: &str) -> Result<StreamRow> {
    let row = stream::update_sync_status(pool, stream_id, status)
        .await
        .context("Failed to update sync status")?;

    tracing::info!(stream_id, status, "Sync status updated");

    Ok(row)
}

/// Deactivate a stream (soft-delete).
pub async fn deactivate_stream(pool: &DbPool, stream_id: i32) -> Result<StreamRow> {
    let row = stream::set_active(pool, stream_id, false)
        .await
        .context("Failed to deactivate stream")?;

    tracing::info!(stream_id, "Stream deactivated");

    Ok(row)
}
