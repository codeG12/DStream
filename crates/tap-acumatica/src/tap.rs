use anyhow::Result;
use chrono::Utc;
use serde_json::json;

use crate::client::AcumaticaClient;
use crate::config::AcumaticaConfig;
use crate::entities::supported_streams;

// ── Discover ────────────────────────────────────────────────────────────

/// Print a JSON catalog of available streams to stdout.
///
/// This does **not** require a live Acumatica connection — it returns
/// the statically-known stream definitions.
pub fn discover() -> Result<()> {
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

/// Authenticate, fetch all records for each supported stream, and write
/// them to stdout as JSONL (one JSON object per line).
///
/// Output format per line:
/// ```json
/// {"type":"RECORD","stream":"business_accounts","record":{...},"time_extracted":"..."}
/// ```
pub fn sync(config: AcumaticaConfig) -> Result<()> {
    let mut client = AcumaticaClient::new(config)?;
    client.login()?;

    let streams = supported_streams();

    for stream in &streams {
        tracing::info!(stream = stream.stream_name, "Starting sync");

        // Emit SCHEMA message
        let schema_msg = json!({
            "type": "SCHEMA",
            "stream": stream.stream_name,
            "key_properties": stream.key_properties,
            "replication_key": stream.replication_key,
        });
        println!("{}", serde_json::to_string(&schema_msg)?);

        // Fetch all records via paginated GET
        let records = client.get_all_entity_records(stream.entity_name)?;

        let now = Utc::now().to_rfc3339();
        for record in &records {
            let msg = json!({
                "type": "RECORD",
                "stream": stream.stream_name,
                "record": record,
                "time_extracted": now,
            });
            println!("{}", serde_json::to_string(&msg)?);
        }

        // Emit STATE with last-modified bookmark
        if let Some(rk) = stream.replication_key {
            if let Some(last) = find_max_field(&records, rk) {
                let state_msg = json!({
                    "type": "STATE",
                    "value": {
                        "bookmarks": {
                            stream.stream_name: {
                                rk: last,
                            }
                        }
                    }
                });
                println!("{}", serde_json::to_string(&state_msg)?);
            }
        }

        tracing::info!(
            stream = stream.stream_name,
            records = records.len(),
            "Sync complete for stream"
        );
    }

    client.logout();
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────────────

/// Find the maximum string value for a given Acumatica-style field
/// across a set of records.
///
/// Acumatica wraps field values like `{ "FieldName": { "value": "..." } }`,
/// so we look inside the `value` wrapper.
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
