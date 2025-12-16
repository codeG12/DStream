use crate::core::http::{HttpClient, HttpResponse};
use crate::core::protocol::Message;
use anyhow::Result;
use arrow::datatypes::SchemaRef;
use async_trait::async_trait;
use futures::stream::BoxStream;
use reqwest;
use reqwest::{header, Body, Method, Client as req_client};
use reqwest::header::HeaderMap;
use serde_json::Value;

pub trait Tap: Send + Sync {}

#[async_trait]
pub trait Discover: Send + Sync {
    async fn discover(&self) -> Result<()>;
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
pub trait TapState: Send + Sync {
    async fn get_state(&self) -> Result<Value>;
    async fn set_state(&mut self, state: Value) -> Result<()> {
        Ok(())
    }
}
pub trait Target: Send + Sync {}
#[async_trait]
pub trait Client: Send + Sync {

    async fn authenticate(&mut self, credentials: Value) -> Result<()>;
    async fn refresh_token(&mut self) -> Result<()>;
    fn get_client(&self) -> &dyn HttpClient;
    async fn request(
        &self,
        url: &str,
        method: Method,
        body: Option<Body>,
        headers: HeaderMap,
        timeout: core::time::Duration,
    ) -> Result<HttpResponse> {
        let client = req_client::builder().timeout(timeout).build()?;

        let mut request_builder = client.request(method, url).headers(headers);

        // Only add body for methods that support it
        if let Some(body) = body {
            request_builder = request_builder.body(body);
        }

        let response = request_builder.send().await?;

        let status = response.status().as_u16();

        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(name, value)| {
                let name_str = name.as_str().to_owned();
                let value_str = value.to_str().unwrap_or("").to_owned();
                (name_str, value_str)
            })
            .collect();

        let body_bytes = response.bytes().await?;
        let body_value = match serde_json::from_slice::<Value>(&body_bytes) {
            Ok(json) => json,
            Err(_) => {
                let text = String::from_utf8_lossy(&body_bytes).to_string();
                Value::String(text)
            }
        };

        Ok(HttpResponse {
            status,
            headers: headers,
            body: body_value,
        })
    }
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
