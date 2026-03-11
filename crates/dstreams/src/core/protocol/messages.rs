use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Core message types for the DStream protocol
#[derive(Debug, Clone)]
pub enum Message {
    Schema(SchemaMessage),
    Record(RecordMessage),
    State(StateMessage),
    Catalog(CatalogMessage),
    Metric(MetricMessage),
}

/// Schema message containing stream schema definition
#[derive(Debug, Clone)]
pub struct SchemaMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Stream name
    pub stream: String,
    /// Arrow schema
    pub schema: SchemaRef,
    /// Primary key properties
    pub key_properties: Vec<String>,
    /// Bookmark properties for incremental extraction
    pub bookmark_properties: Vec<String>,
    /// Timestamp when schema was captured
    pub timestamp: DateTime<Utc>,
}

/// Record message containing actual data
#[derive(Debug, Clone)]
pub struct RecordMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Stream name
    pub stream: String,
    /// Record batch containing the data
    pub record: RecordBatch,
    /// When the data was extracted
    pub time_extracted: DateTime<Utc>,
    /// Sequence number for ordering
    pub sequence: Option<u64>,
}

/// State message for checkpointing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMessage {
    /// Unique message ID
    pub id: Uuid,
    /// State value (typically contains bookmarks)
    pub value: Value,
    /// Timestamp when state was captured
    pub timestamp: DateTime<Utc>,
}

/// Catalog message from discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Catalog data
    pub catalog: Value,
    /// Timestamp when catalog was generated
    pub timestamp: DateTime<Utc>,
}

/// Metric message for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Metric type (e.g., "record_count", "http_request")
    pub metric_type: MetricType,
    /// Metric value
    pub value: f64,
    /// Associated stream (if applicable)
    pub stream: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Timestamp when metric was recorded
    pub timestamp: DateTime<Utc>,
}

/// Types of metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    RecordCount,
    HttpRequest,
    BytesProcessed,
    ProcessingTime,
    ErrorCount,
    Custom(String),
}

// Builder implementations for ergonomic message construction

impl SchemaMessage {
    pub fn new(stream: String, schema: SchemaRef) -> Self {
        Self {
            id: Uuid::new_v4(),
            stream,
            schema,
            key_properties: Vec::new(),
            bookmark_properties: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn with_key_properties(mut self, keys: Vec<String>) -> Self {
        self.key_properties = keys;
        self
    }

    pub fn with_bookmark_properties(mut self, bookmarks: Vec<String>) -> Self {
        self.bookmark_properties = bookmarks;
        self
    }
}

impl RecordMessage {
    pub fn new(stream: String, record: RecordBatch) -> Self {
        Self {
            id: Uuid::new_v4(),
            stream,
            record,
            time_extracted: Utc::now(),
            sequence: None,
        }
    }

    /// Set sequence number
    pub fn with_sequence(mut self, seq: u64) -> Self {
        self.sequence = Some(seq);
        self
    }

    pub fn row_count(&self) -> usize {
        self.record.num_rows()
    }
}

impl StateMessage {
    /// Create a new state message
    pub fn new(value: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
            timestamp: Utc::now(),
        }
    }
}

impl CatalogMessage {
    /// Create a new catalog message
    pub fn new(catalog: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            catalog,
            timestamp: Utc::now(),
        }
    }
}

impl MetricMessage {
    /// Create a new metric message
    pub fn new(metric_type: MetricType, value: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            metric_type,
            value,
            stream: None,
            tags: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Set associated stream
    pub fn with_stream(mut self, stream: String) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add a single tag
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
}

impl Message {
    /// Get the message type as a string
    pub fn message_type(&self) -> &'static str {
        match self {
            Message::Schema(_) => "SCHEMA",
            Message::Record(_) => "RECORD",
            Message::State(_) => "STATE",
            Message::Catalog(_) => "CATALOG",
            Message::Metric(_) => "METRIC",
        }
    }

    pub fn is_schema(&self) -> bool {
        matches!(self, Message::Schema(_))
    }

    pub fn is_record(&self) -> bool {
        matches!(self, Message::Record(_))
    }

    pub fn is_state(&self) -> bool {
        matches!(self, Message::State(_))
    }
}
