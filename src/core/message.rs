use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::core::errors::{NatsError, Result};

/// Wire-format message envelope for NATS tap → target communication.
///
/// Matches the spec from `Docs/Impl Plan/index.md`:
/// ```json
/// {
///   "id": "ulid",
///   "source": "tap_postgres",
///   "stream_name": "users",
///   "schema_version": "1.0.0",
///   "created_at": "2026-03-08T12:05:00Z",
///   "data": [...],
///   "state": {},
///   "trace_id": "uuid-backbone-trace-123"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEnvelope {
    /// Unique message ID (ULID)
    pub id: String,

    /// Identifier of the source tap (e.g. `"tap_postgres"`)
    pub source: String,

    /// Logical stream / table name (e.g. `"users"`)
    pub stream_name: String,

    /// Schema version tag
    pub schema_version: String,

    /// Timestamp when the envelope was created
    pub created_at: DateTime<Utc>,

    /// Batch of data records
    pub data: Vec<Value>,

    /// Current bookmark / state snapshot
    pub state: Value,

    /// Distributed-tracing correlation ID
    pub trace_id: String,
}

impl StreamEnvelope {
    /// Build a new envelope with auto-generated `id`, `created_at`, and `trace_id`.
    pub fn new(
        source: String,
        stream_name: String,
        data: Vec<Value>,
        state: Value,
    ) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            source,
            stream_name,
            schema_version: "1.0.0".to_string(),
            created_at: Utc::now(),
            data,
            state,
            trace_id: Uuid::new_v4().to_string(),
        }
    }

    /// Build a new envelope with a custom schema version.
    pub fn with_schema_version(mut self, version: String) -> Self {
        self.schema_version = version;
        self
    }

    /// Serialize to JSON bytes for NATS publishing.
    pub fn to_bytes(&self) -> Result<bytes::Bytes> {
        let json = serde_json::to_vec(self)
            .map_err(|e| NatsError::DeserializeFailed(e.to_string()))?;
        Ok(bytes::Bytes::from(json))
    }

    /// Deserialize from raw bytes received from NATS.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let envelope = serde_json::from_slice(data)
            .map_err(|e| NatsError::DeserializeFailed(e.to_string()))?;
        Ok(envelope)
    }

    /// Build the NATS subject for a given stream-id + table-name.
    ///
    /// Format: `ETL.data.<stream_id>.<table_name>`
    pub fn subject(stream_id: i32, table_name: &str) -> String {
        format!("ETL.data.{}.{}", stream_id, table_name)
    }
}
