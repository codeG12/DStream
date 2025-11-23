use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Message {
    /// A schema definition message.
    Schema(SchemaRef),
    /// A data record message containing a batch of rows.
    Record(RecordBatch),
    /// A state message for checkpointing.
    State(Value),
}
