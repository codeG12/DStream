use crate::core::errors::{NatsError, Result};

/// Configuration for connecting to a NATS server.
#[derive(Debug, Clone)]
pub struct NatsConfig {
    /// NATS server URL (e.g. `nats://localhost:4222`)
    pub server_url: String,
}

impl Default for NatsConfig {
    fn default() -> Self {
        Self {
            server_url: "nats://localhost:4222".to_string(),
        }
    }
}

/// Thin wrapper around the async-nats client.
#[derive(Clone)]
pub struct NatsClient {
    client: async_nats::Client,
}

impl NatsClient {
    /// Connect to the NATS server described by `config`.
    pub async fn connect(config: &NatsConfig) -> Result<Self> {
        let client = async_nats::connect(&config.server_url)
            .await
            .map_err(|e| NatsError::ConnectionFailed(e.to_string()))?;

        tracing::info!(url = %config.server_url, "Connected to NATS");

        Ok(Self { client })
    }

    /// Publish raw bytes to a subject.
    pub async fn publish(&self, subject: &str, payload: bytes::Bytes) -> Result<()> {
        self.client
            .publish(subject.to_string(), payload)
            .await
            .map_err(|e| NatsError::PublishFailed(e.to_string()))?;

        Ok(())
    }

    /// Subscribe to a subject and return the message stream.
    pub async fn subscribe(
        &self,
        subject: &str,
    ) -> Result<async_nats::Subscriber> {
        let subscriber = self
            .client
            .subscribe(subject.to_string())
            .await
            .map_err(|e| NatsError::SubscribeFailed(e.to_string()))?;

        tracing::info!(subject, "Subscribed to NATS subject");

        Ok(subscriber)
    }

    /// Flush pending published messages to the server.
    pub async fn flush(&self) -> Result<()> {
        self.client
            .flush()
            .await
            .map_err(|e| NatsError::PublishFailed(format!("flush failed: {e}")))?;

        Ok(())
    }
}
