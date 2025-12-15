use arrow::datatypes::SchemaRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub streams: Vec<CatalogEntry>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CatalogMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogMetadata {
    pub generated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_version: Option<String>,

    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub stream: String,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_stream_id: Option<String>,

    #[serde(skip)]
    pub schema: Option<SchemaRef>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_json: Option<Value>,

    pub metadata: StreamMetadata,

    #[serde(default)]
    pub key_properties: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_key: Option<String>,

    pub replication_method: ReplicationMethod,

    #[serde(default = "default_true")]
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    #[serde(default)]
    pub is_view: bool,

    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReplicationMethod {
    FullTable,

    Incremental,

    /// Log-based replication (CDC)
    LogBased,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            streams: Vec::new(),
            metadata: Some(CatalogMetadata {
                generated_at: Utc::now(),
                tap_version: None,
                properties: HashMap::new(),
            }),
        }
    }

    pub fn add_stream(&mut self, entry: CatalogEntry) {
        self.streams.push(entry);
    }
    pub fn get_stream(&self, name: &str) -> Option<&CatalogEntry> {
        self.streams.iter().find(|s| s.stream == name)
    }

    pub fn selected_streams(&self) -> Vec<&CatalogEntry> {
        self.streams.iter().filter(|s| s.selected).collect()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalogEntry {
    /// Create a new catalog entry
    pub fn new(stream: String, replication_method: ReplicationMethod) -> Self {
        Self {
            stream,
            tap_stream_id: None,
            schema: None,
            schema_json: None,
            metadata: StreamMetadata::default(),
            key_properties: Vec::new(),
            replication_key: None,
            replication_method,
            selected: true,
        }
    }
    pub fn with_schema(mut self, schema: SchemaRef) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_key_properties(mut self, keys: Vec<String>) -> Self {
        self.key_properties = keys;
        self
    }
    pub fn with_replication_key(mut self, key: String) -> Self {
        self.replication_key = Some(key);
        self
    }
    pub fn with_metadata(mut self, metadata: StreamMetadata) -> Self {
        self.metadata = metadata;
        self
    }
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

fn default_true() -> bool {
    true
}

