use anyhow::{Context, Result};

use crate::core::protocol::catalog::Catalog;
use crate::core::config::{TapConfig, TargetConfig};
use crate::dal::models::UpsertState;
use crate::dal::DbPool;
use crate::dal::{catalog as catalog_dal, state as state_dal, stream};

/// Run the full sync pipeline for a given stream.
///
/// Orchestrates:
/// 1. Load stream metadata from DB
/// 2. Load state bookmarks for resumable sync
/// 3. (Future) Extract data via tap
/// 4. (Future) Load data via target
/// 5. Persist updated state bookmarks
/// 6. Mark stream sync status
pub async fn run_sync(
    pool: &DbPool,
    stream_id: i32,
    tap_config: &TapConfig,
    target_config: &TargetConfig,
) -> Result<()> {
    // 1. Validate configs
    tap_config.validate().context("Invalid tap configuration")?;
    target_config
        .validate()
        .context("Invalid target configuration")?;

    // 2. Load stream from DB
    let stream_row = stream::get_by_id(pool, stream_id)
        .await
        .context("Database query failed")?
        .context("Stream not found")?;

    tracing::info!(
        stream_id = stream_row.stream_id,
        name = %stream_row.stream_name,
        "Starting sync"
    );

    // 3. Load existing state bookmarks
    let state_rows = state_dal::list_by_stream(pool, stream_id)
        .await
        .context("Failed to load state")?;

    tracing::info!(bookmarks = state_rows.len(), "Loaded state bookmarks");

    // 4. Load catalog entries for the source connector
    let catalog_entries = catalog_dal::list_by_connector(pool, stream_row.source_connector_id)
        .await
        .context("Failed to load catalog")?;

    let selected_count = catalog_entries
        .iter()
        .filter(|e| e.is_selected.unwrap_or(false))
        .count();

    tracing::info!(
        total = catalog_entries.len(),
        selected = selected_count,
        "Loaded catalog entries"
    );

    // ── Sync execution placeholder ──────────────────────────────────────
    // TODO: Implement actual tap extraction → target loading pipeline.
    // For each selected catalog entry:
    //   - Extract data from source using tap connector
    //   - Transform data as needed
    //   - Write data to target using target connector
    //   - Update state bookmarks
    tracing::warn!("Sync execution pending — tap and target connectors need implementation");

    // 5. Mark sync complete
    stream::update_sync_status(pool, stream_id, "completed")
        .await
        .context("Failed to update sync status")?;

    tracing::info!(stream_id, "Sync completed");

    Ok(())
}

/// Persist a state bookmark for a specific table within a stream.
pub async fn save_state_bookmark(
    pool: &DbPool,
    stream_id: i32,
    table_name: String,
    bookmark_column: Option<String>,
    bookmark_value: Option<String>,
    bookmark_type: Option<String>,
    records_synced: Option<i64>,
) -> Result<()> {
    let input = UpsertState {
        stream_id,
        table_name: table_name.clone(),
        bookmark_column,
        bookmark_value,
        bookmark_type,
        records_synced,
    };

    state_dal::upsert(pool, input)
        .await
        .context("Failed to upsert state bookmark")?;

    tracing::debug!(
        stream_id,
        table = %table_name,
        "State bookmark saved"
    );

    Ok(())
}
