use arrow::datatypes::SchemaRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Represents a catalog of available streams from a data source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    /// List of available streams
    pub streams: Vec<CatalogEntry>,

    /// Catalog metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CatalogMetadata>,
}

/// Metadata about the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogMetadata {
    /// When the catalog was generated
    pub generated_at: DateTime<Utc>,

    /// Version of the tap that generated this catalog
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_version: Option<String>,

    /// Additional metadata
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

/// Represents a single stream in the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    /// Unique identifier for the stream
    pub stream: String,

    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_stream_id: Option<String>,

    /// Schema definition (serialized as JSON)
    #[serde(skip)]
    pub schema: Option<SchemaRef>,

    /// JSON representation of the schema for serialization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_json: Option<Value>,

    /// Stream metadata
    pub metadata: StreamMetadata,

    /// List of key properties (primary keys)
    #[serde(default)]
    pub key_properties: Vec<String>,

    /// Replication key for incremental extraction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_key: Option<String>,

    /// Replication method
    pub replication_method: ReplicationMethod,

    /// Whether this stream is selected for extraction
    #[serde(default = "default_true")]
    pub selected: bool,
}

/// Metadata about a stream
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamMetadata {
    /// Estimated row count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<u64>,

    /// Database or schema name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,

    /// Table name (for database sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    /// Whether the stream is a view
    #[serde(default)]
    pub is_view: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

/// Replication method for a stream
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReplicationMethod {
    /// Full table replication (extract all data every time)
    FullTable,

    /// Incremental replication using a replication key
    Incremental,

    /// Log-based replication (CDC)
    LogBased,
}

impl Catalog {
    /// Create a new empty catalog
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

    /// Add a stream to the catalog
    pub fn add_stream(&mut self, entry: CatalogEntry) {
        self.streams.push(entry);
    }

    /// Get a stream by name
    pub fn get_stream(&self, name: &str) -> Option<&CatalogEntry> {
        self.streams.iter().find(|s| s.stream == name)
    }

    /// Get all selected streams
    pub fn selected_streams(&self) -> Vec<&CatalogEntry> {
        self.streams.iter().filter(|s| s.selected).collect()
    }

    /// Load catalog from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize catalog to JSON string
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

    /// Set the schema for this stream
    pub fn with_schema(mut self, schema: SchemaRef) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Set key properties
    pub fn with_key_properties(mut self, keys: Vec<String>) -> Self {
        self.key_properties = keys;
        self
    }

    /// Set replication key
    pub fn with_replication_key(mut self, key: String) -> Self {
        self.replication_key = Some(key);
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: StreamMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Mark as selected or not
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_creation() {
        let mut catalog = Catalog::new();
        assert_eq!(catalog.streams.len(), 0);
        assert!(catalog.metadata.is_some());

        let entry = CatalogEntry::new("users".to_string(), ReplicationMethod::Incremental)
            .with_key_properties(vec!["id".to_string()])
            .with_replication_key("updated_at".to_string());

        catalog.add_stream(entry);
        assert_eq!(catalog.streams.len(), 1);
    }

    #[test]
    fn test_catalog_entry_builder() {
        let entry = CatalogEntry::new("orders".to_string(), ReplicationMethod::FullTable)
            .with_key_properties(vec!["order_id".to_string()])
            .with_selected(true);

        assert_eq!(entry.stream, "orders");
        assert_eq!(entry.replication_method, ReplicationMethod::FullTable);
        assert_eq!(entry.key_properties, vec!["order_id"]);
        assert!(entry.selected);
    }

    #[test]
    fn test_selected_streams() {
        let mut catalog = Catalog::new();

        catalog.add_stream(
            CatalogEntry::new("stream1".to_string(), ReplicationMethod::Incremental)
                .with_selected(true),
        );
        catalog.add_stream(
            CatalogEntry::new("stream2".to_string(), ReplicationMethod::FullTable)
                .with_selected(false),
        );
        catalog.add_stream(
            CatalogEntry::new("stream3".to_string(), ReplicationMethod::Incremental)
                .with_selected(true),
        );

        let selected = catalog.selected_streams();
        assert_eq!(selected.len(), 2);
    }

    #[test]
    fn test_replication_method_serialization() {
        let method = ReplicationMethod::Incremental;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"INCREMENTAL\"");

        let deserialized: ReplicationMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ReplicationMethod::Incremental);
    }

    #[test]
    fn test_catalog_json_roundtrip() {
        let mut catalog = Catalog::new();
        catalog.add_stream(
            CatalogEntry::new("test_stream".to_string(), ReplicationMethod::Incremental)
                .with_key_properties(vec!["id".to_string()]),
        );

        let json = catalog.to_json().unwrap();
        let deserialized = Catalog::from_json(&json).unwrap();

        assert_eq!(deserialized.streams.len(), 1);
        assert_eq!(deserialized.streams[0].stream, "test_stream");
    }
}
