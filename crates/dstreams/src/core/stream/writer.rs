use serde_json::Value;

use crate::core::errors::Result;
use crate::core::stream::message::StreamEnvelope;
use crate::core::stream::nats::NatsClient;

/// Tap-side publisher that writes data batches to NATS.
pub struct StreamWriter {
    nats: NatsClient,
}

impl StreamWriter {
    pub fn new(nats: NatsClient) -> Self {
        Self { nats }
    }

    /// Publish a data batch for a given stream + table.
    ///
    /// Builds a [`StreamEnvelope`], serializes it, and publishes to
    /// `ETL.data.<stream_id>.<table_name>`.
    pub async fn write(
        &self,
        stream_id: i32,
        table_name: &str,
        source: &str,
        data: Vec<Value>,
        state: Value,
    ) -> Result<()> {
        let subject = StreamEnvelope::subject(stream_id, table_name);

        let envelope = StreamEnvelope::new(
            source.to_string(),
            table_name.to_string(),
            data,
            state,
        );

        let payload = envelope.to_bytes()?;
        let payload_len = payload.len();

        self.nats.publish(&subject, payload).await?;

        tracing::info!(
            subject = %subject,
            message_id = %envelope.id,
            records = envelope.data.len(),
            bytes = payload_len,
            "Published stream envelope"
        );

        Ok(())
    }

    /// Flush any buffered publishes to the NATS server.
    pub async fn flush(&self) -> Result<()> {
        self.nats.flush().await
    }
}
