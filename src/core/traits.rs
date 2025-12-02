use crate::core::http::{HttpClient, HttpRequest, HttpResponse};
use crate::core::pagination::Page;
use crate::core::protocol::Message;
use anyhow::Result;
use arrow::datatypes::SchemaRef;
use async_trait::async_trait;
use futures::stream::BoxStream;
use serde_json::Value;

pub trait Tap: Send + Sync {}

#[async_trait]
pub trait Discover: Send + Sync {
    async fn discover(&self) -> Result<()>;
}

#[async_trait]
pub trait TapClient: Send + Sync {
    fn get_client(&self) -> &dyn HttpClient;

    async fn request(&self, req: HttpRequest) -> Result<HttpResponse>;
}
#[async_trait]
pub trait TapStream: Send + Sync {
    async fn stream(&mut self) -> BoxStream<'_, Result<Message>>;
}
#[async_trait]
pub trait TapSync: Send + Sync {
    async fn sync(&mut self) -> Result<Vec<Message>>;
}
#[async_trait]
pub trait Pagination: Send + Sync {
    async fn next_page(&mut self) -> Result<Option<Page>>;
    fn has_more(&self) -> bool;
}
#[async_trait]
pub trait TapAuth: Send + Sync {
    async fn authenticate(&mut self, credentials: Value) -> Result<()>;
    async fn refresh_token(&mut self) -> Result<()>;
}
#[async_trait]
pub trait TapState: Send + Sync {
    async fn get_state(&self) -> Result<Value>;
    async fn set_state(&mut self, state: Value) -> Result<()>;
}
pub trait Target: Send + Sync {}
#[async_trait]
pub trait TargetClient: Send + Sync {
    fn get_client(&self) -> &dyn HttpClient;
    async fn request(&self, req: HttpRequest) -> Result<HttpResponse>;
}

#[async_trait]
pub trait TargetStream: Send + Sync {
    async fn stream_write(&mut self, stream: BoxStream<'_, Result<Message>>) -> Result<()>;
}

#[async_trait]
pub trait TargetSync: Send + Sync {
    async fn sync_write(&mut self, messages: Vec<Message>) -> Result<()>;
}

#[async_trait]
pub trait BatchSink: Send + Sync {
    async fn begin_batch(&mut self) -> Result<()>;

    async fn write_to_batch(&mut self, message: Message) -> Result<()>;

    async fn commit_batch(&mut self) -> Result<()>;

    async fn rollback_batch(&mut self) -> Result<()>;
}

#[async_trait]
pub trait StreamSink: Send + Sync {
    async fn write(&mut self, message: Message) -> Result<()>;
}

#[async_trait]
pub trait TargetAuth: Send + Sync {
    async fn authenticate(&mut self, credentials: Value) -> Result<()>;

    async fn refresh_token(&mut self) -> Result<()>;
}

#[async_trait]
pub trait TargetState: Send + Sync {
    async fn get_state(&self) -> Result<Value>;

    async fn set_state(&mut self, state: Value) -> Result<()>;
}

#[async_trait]
pub trait Transform: Send + Sync {
    async fn transform(&self, message: Message) -> Result<Message>;

    fn transform_schema(&self, schema: SchemaRef) -> Result<SchemaRef>;
}

#[async_trait]
pub trait Sink: Send + Sync {
    async fn initialize(&mut self) -> Result<()>;

    async fn finalize(&mut self) -> Result<()>;
}
