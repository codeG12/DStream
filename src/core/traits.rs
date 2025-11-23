use crate::core::protocol::Message;
use anyhow::Result;
use arrow::datatypes::SchemaRef;
use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
pub trait Tap: Send + Sync {
    /// Discover the schema of the source.
    async fn discover(&self) -> Result<SchemaRef>;

    /// Read data from the source as a stream of messages.
    async fn read(&mut self) -> BoxStream<'_, Result<Message>>;
}

#[async_trait]
pub trait Target: Send + Sync {
    /// Write a single message to the destination.
    async fn write(&mut self, message: Message) -> Result<()>;
}
