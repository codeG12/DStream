use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

// ─── Row types (mirror DB tables) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConnectorRow {
    pub connector_id: i32,
    pub connector_name: String,
    pub connector_type: String,
    pub config: Value,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StreamRow {
    pub stream_id: i32,
    pub stream_name: String,
    pub source_connector_id: i32,
    pub target_connector_id: i32,
    pub is_active: Option<bool>,
    pub last_sync_status: Option<String>,
    pub last_sync_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CatalogRow {
    pub catalog_id: i32,
    pub connector_id: i32,
    pub table_name: String,
    pub schema_name: Option<String>,
    pub table_schema: Value,
    pub key_properties: Option<Value>,
    pub replication_method: Option<String>,
    pub replication_key: Option<String>,
    pub is_selected: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StreamConfigurationRow {
    pub id: i32,
    pub stream_id: i32,
    pub catalog_item_id: i32,
    pub is_selected: Option<bool>,
    pub replication_method: Option<String>,
    pub replication_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StateRow {
    pub stream_id: i32,
    pub table_name: String,
    pub bookmark_column: Option<String>,
    pub bookmark_value: Option<String>,
    pub bookmark_type: Option<String>,
    pub records_synced: Option<i64>,
    pub last_sync_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// ─── Input DTOs ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CreateConnector {
    pub connector_name: String,
    pub connector_type: String,
    pub config: Value,
}

#[derive(Debug, Clone)]
pub struct CreateStream {
    pub stream_name: String,
    pub source_connector_id: i32,
    pub target_connector_id: i32,
}

#[derive(Debug, Clone)]
pub struct CreateCatalogEntry {
    pub connector_id: i32,
    pub table_name: String,
    pub schema_name: Option<String>,
    pub table_schema: Value,
    pub key_properties: Option<Value>,
    pub replication_method: Option<String>,
    pub replication_key: Option<String>,
    pub is_selected: bool,
}

#[derive(Debug, Clone)]
pub struct CreateStreamConfiguration {
    pub stream_id: i32,
    pub catalog_item_id: i32,
    pub is_selected: bool,
    pub replication_method: String,
    pub replication_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpsertState {
    pub stream_id: i32,
    pub table_name: String,
    pub bookmark_column: Option<String>,
    pub bookmark_value: Option<String>,
    pub bookmark_type: Option<String>,
    pub records_synced: Option<i64>,
}
