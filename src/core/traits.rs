use crate::core::http::{HttpClient, HttpRequest, HttpResponse};
use crate::core::pagination::Page;
use crate::core::protocol::Message;
use anyhow::Result;
use arrow::datatypes::SchemaRef;
use async_trait::async_trait;
use futures::stream::BoxStream;
use serde_json::Value;

// ============================================================================
// TAP TRAITS
// ============================================================================

/// Marker trait for Tap (source) connectors
pub trait Tap: Send + Sync {}

/// Schema discovery - discovers and writes catalog to JSON file
#[async_trait]
pub trait Discover: Send + Sync {
    /// Discover the schema and write catalog to file
    async fn discover(&self) -> Result<()>;
}

/// HTTP client capabilities for taps
#[async_trait]
pub trait TapClient: Send + Sync {
    /// Get reference to the HTTP client
    fn get_client(&self) -> &dyn HttpClient;

    /// Make an HTTP request
    async fn request(&self, req: HttpRequest) -> Result<HttpResponse>;
}

/// Streaming data from source
#[async_trait]
pub trait TapStream: Send + Sync {
    /// Stream data from the source
    async fn stream(&mut self) -> BoxStream<'_, Result<Message>>;
}

/// Synchronous reading from source
#[async_trait]
pub trait TapSync: Send + Sync {
    /// Read all data synchronously
    async fn sync(&mut self) -> Result<Vec<Message>>;
}

/// Pagination support for taps
#[async_trait]
pub trait Pagination: Send + Sync {
    /// Fetch the next page of data
    async fn next_page(&mut self) -> Result<Option<Page>>;

    /// Check if more pages are available
    fn has_more(&self) -> bool;
}

/// Authentication for taps
#[async_trait]
pub trait TapAuth: Send + Sync {
    /// Authenticate with the source
    async fn authenticate(&mut self, credentials: Value) -> Result<()>;

    /// Refresh authentication token
    async fn refresh_token(&mut self) -> Result<()>;
}

/// State management for taps
#[async_trait]
pub trait TapState: Send + Sync {
    /// Get current state
    async fn get_state(&self) -> Result<Value>;

    /// Set state
    async fn set_state(&mut self, state: Value) -> Result<()>;
}

// ============================================================================
// TARGET TRAITS
// ============================================================================

/// Marker trait for Target (destination) connectors
pub trait Target: Send + Sync {}

/// HTTP client capabilities for targets
#[async_trait]
pub trait TargetClient: Send + Sync {
    /// Get reference to the HTTP client
    fn get_client(&self) -> &dyn HttpClient;

    /// Make an HTTP request
    async fn request(&self, req: HttpRequest) -> Result<HttpResponse>;
}

/// Streaming writes to target
#[async_trait]
pub trait TargetStream: Send + Sync {
    /// Write a stream of messages to the target
    async fn stream_write(&mut self, stream: BoxStream<'_, Result<Message>>) -> Result<()>;
}

/// Synchronous writes to target
#[async_trait]
pub trait TargetSync: Send + Sync {
    /// Write messages synchronously
    async fn sync_write(&mut self, messages: Vec<Message>) -> Result<()>;
}

/// Batch/transactional writes
#[async_trait]
pub trait BatchSink: Send + Sync {
    /// Begin a new batch transaction
    async fn begin_batch(&mut self) -> Result<()>;

    /// Write a message to the current batch
    async fn write_to_batch(&mut self, message: Message) -> Result<()>;

    /// Commit the current batch
    async fn commit_batch(&mut self) -> Result<()>;

    /// Rollback the current batch
    async fn rollback_batch(&mut self) -> Result<()>;
}

/// Individual message writes
#[async_trait]
pub trait StreamSink: Send + Sync {
    /// Write a single message
    async fn write(&mut self, message: Message) -> Result<()>;
}

/// Authentication for targets
#[async_trait]
pub trait TargetAuth: Send + Sync {
    /// Authenticate with the target
    async fn authenticate(&mut self, credentials: Value) -> Result<()>;

    /// Refresh authentication token
    async fn refresh_token(&mut self) -> Result<()>;
}

/// State management for targets
#[async_trait]
pub trait TargetState: Send + Sync {
    /// Get current state
    async fn get_state(&self) -> Result<Value>;

    /// Set state
    async fn set_state(&mut self, state: Value) -> Result<()>;
}

/// Message transformation
#[async_trait]
pub trait Transform: Send + Sync {
    /// Transform a message
    async fn transform(&self, message: Message) -> Result<Message>;

    /// Transform a schema
    fn transform_schema(&self, schema: SchemaRef) -> Result<SchemaRef>;
}

/// Base sink trait with lifecycle methods
#[async_trait]
pub trait Sink: Send + Sync {
    /// Initialize the sink
    async fn initialize(&mut self) -> Result<()>;

    /// Finalize and cleanup the sink
    async fn finalize(&mut self) -> Result<()>;
}
