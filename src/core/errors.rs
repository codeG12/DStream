use thiserror::Error;

/// Main error type for DStream operations
#[derive(Error, Debug)]
pub enum DStreamError {
    /// Errors related to tap (source) operations
    #[error("Tap error: {0}")]
    Tap(#[from] TapError),

    /// Errors related to target (sink) operations
    #[error("Target error: {0}")]
    Target(#[from] TargetError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// State management errors
    #[error("State error: {0}")]
    State(#[from] StateError),

    /// Protocol/message errors
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Arrow-related errors
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    /// Generic error with custom message
    #[error("{0}")]
    Custom(String),
}

/// Errors specific to tap operations
#[derive(Error, Debug)]
pub enum TapError {
    #[error("Failed to discover schema: {0}")]
    DiscoveryFailed(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Failed to fetch data: {0}")]
    FetchFailed(String),

    #[error("Pagination error: {0}")]
    PaginationError(String),

    #[error("Invalid stream: {0}")]
    InvalidStream(String),

    #[error("HTTP request failed: {0}")]
    HttpError(String),
}

/// Errors specific to target operations
#[derive(Error, Debug)]
pub enum TargetError {
    #[error("Failed to write data: {0}")]
    WriteFailed(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),

    #[error("Batch operation failed: {0}")]
    BatchFailed(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Transformation error: {0}")]
    TransformError(String),
}

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid configuration value for '{field}': {reason}")]
    InvalidValue { field: String, reason: String },

    #[error("Failed to load configuration from {path}: {reason}")]
    LoadFailed { path: String, reason: String },

    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// State management errors
#[derive(Error, Debug)]
pub enum StateError {
    #[error("Failed to load state from {path}: {reason}")]
    LoadFailed { path: String, reason: String },

    #[error("Failed to save state to {path}: {reason}")]
    SaveFailed { path: String, reason: String },

    #[error("Invalid state format: {0}")]
    InvalidFormat(String),

    #[error("State merge conflict: {0}")]
    MergeConflict(String),

    #[error("Bookmark not found for stream: {0}")]
    BookmarkNotFound(String),
}

/// Protocol and message errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid message type: expected {expected}, got {actual}")]
    InvalidMessageType { expected: String, actual: String },

    #[error("Missing required field in message: {0}")]
    MissingField(String),

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("Invalid catalog: {0}")]
    InvalidCatalog(String),

    #[error("Message serialization failed: {0}")]
    SerializationFailed(String),
}

/// Type alias for Results using DStreamError
pub type Result<T> = std::result::Result<T, DStreamError>;
