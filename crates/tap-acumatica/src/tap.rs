use anyhow::{Context, Result};
use chrono::Utc;
use serde_json::json;

use dstreams::core::db;
use dstreams::core::stream::nats::{NatsClient, NatsConfig};
use dstreams::core::stream::writer::StreamWriter;
use dstreams::dal::models::{CreateStream, CreateStreamConfiguration, CatalogRow};

use crate::client::AcumaticaClient;
use crate::config::AcumaticaConfig;
use crate::entities::supported_streams;

// ── Discover ────────────────────────────────────────────────────────────

/// Print a JSON catalog of available streams to stdout.
///
/// This does **not** require a live Acumatica connection — it returns
/// the statically-known stream definitions.
pub async fn discover() -> Result<()> {
    let streams = supported_streams();

    let entries: Vec<serde_json::Value> = streams
        .iter()
        .map(|s| {
            json!({
                "stream": s.stream_name,
                "tap_stream_id": s.entity_name,
                "replication_method": if s.replication_key.is_some() {
                    "INCREMENTAL"
                } else {
                    "FULL_TABLE"
                },
                "key_properties": s.key_properties,
                "replication_key": s.replication_key,
                "selected": true,
                "metadata": {
                    "table_name": s.entity_name,
                }
            })
        })
        .collect();

    let catalog = json!({
        "streams": entries,
        "metadata": {
            "generated_at": Utc::now().to_rfc3339(),
            "tap_version": env!("CARGO_PKG_VERSION"),
        }
    });

    let output = serde_json::to_string_pretty(&catalog)?;
    println!("{}", output);
    Ok(())
}

// ── Sync ────────────────────────────────────────────────────────────────

/// Authenticate, fetch all records for each configured stream, write to stream_configuration,
/// and publish to NATS.
pub async fn sync(config: AcumaticaConfig) -> Result<()> {
    // 1. Database Connection
    let pool = db::client().await.context("Failed to connect to Postgres")?;
    
    // 2. NATS Connection
    let nats_cfg = NatsConfig::default();
    let nats_client = NatsClient::connect(&nats_cfg).await.context("Failed to connect to NATS")?;
    let writer = StreamWriter::new(nats_client);

    let mut acumatica_client = AcumaticaClient::new(config.clone())?;
    acumatica_client.login()?;

    // Assuming we have dummy connector IDs or they exist (e.g. 1 and 2)
    // Create a new stream
    let stream_name = format!("acumatica_sync_{}", Utc::now().timestamp());
    let stream = dstreams::dal::stream::create(&pool, CreateStream {
        stream_name: stream_name.clone(),
        source_connector_id: 1, // Placeholder
        target_connector_id: 2, // Placeholder
    }).await.context("Failed to create stream entry")?;
    
    tracing::info!(stream_id = stream.stream_id, stream_name = %stream.stream_name, "Created stream record");

    for (table_name, table_cfg) in &config.tables {
        tracing::info!(table_name = %table_name, "Processing configured table");

        // Validate against Catalog
        let catalog_item = sqlx::query_as::<_, CatalogRow>("SELECT * FROM catalog WHERE table_name = $1")
            .bind(table_name)
            .fetch_optional(&pool)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Table '{}' not found in Postgres catalog", table_name))?;

        // Create Stream Configuration
        dstreams::dal::stream_configuration::create(&pool, CreateStreamConfiguration {
            stream_id: stream.stream_id,
            catalog_item_id: catalog_item.catalog_id,
            is_selected: true,
            replication_method: table_cfg.replication_method.clone(),
            replication_key: if table_cfg.valid_replication_keys.is_empty() { None } else { Some(table_cfg.valid_replication_keys.clone()) },
        }).await.context("Failed to create stream_configuration entry")?;
        
        // Fetch data from Acumatica
        let records = acumatica_client.get_all_entity_records(table_name)?;
        
        // Prepare state (bookmark)
        let mut state = json!({});
        if !table_cfg.valid_replication_keys.is_empty() {
            if let Some(last) = find_max_field(&records, &table_cfg.valid_replication_keys) {
                state = json!({
                    "bookmarks": {
                        table_name: {
                            &table_cfg.valid_replication_keys: last
                        }
                    }
                });
            }
        }

        // Publish to NATS in chunks (or all at once if small enough)
        // StreamEnvelope expects a Vec<Value>
        writer.write(
            stream.stream_id,
            table_name,
            "acumatica",
            records.clone(),
            state,
        ).await.context("Failed to write to NATS")?;
        
        tracing::info!(
            table_name = %table_name,
            records = records.len(),
            "Sync complete for table"
        );
    }
    
    writer.flush().await?;
    acumatica_client.logout();
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────────────

/// Find the maximum string value for a given Acumatica-style field
/// across a set of records.
fn find_max_field(records: &[serde_json::Value], field: &str) -> Option<String> {
    records
        .iter()
        .filter_map(|r| {
            r.get(field)
                .and_then(|f| f.get("value"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .max()
}
