use crate::core::errors::{NatsError, Result};

#[derive(Debug, Clone)]
pub struct NatsConfig {
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
    pub async fn connect(config: &NatsConfig) -> Result<Self> {
        let client = async_nats::connect(&config.server_url)
            .await
            .map_err(|e| NatsError::ConnectionFailed(e.to_string()))?;

        tracing::info!(url = %config.server_url, "Connected to NATS");

        Ok(Self { client })
    }

    pub async fn publish(&self, subject: &str, payload: bytes::Bytes) -> Result<()> {
        self.client
            .publish(subject.to_string(), payload)
            .await
            .map_err(|e| NatsError::PublishFailed(e.to_string()))?;

        Ok(())
    }

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

    pub async fn flush(&self) -> Result<()> {
        self.client
            .flush()
            .await
            .map_err(|e| NatsError::PublishFailed(format!("flush failed: {e}")))?;

        Ok(())
    }
}
