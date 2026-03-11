use futures::StreamExt;

use crate::core::errors::Result;
use crate::core::stream::message::StreamEnvelope;
use crate::core::stream::nats::NatsClient;

/// Target-side consumer that listens for data on a NATS subject.
pub struct StreamListener {
    nats: NatsClient,
}

impl StreamListener {
    pub fn new(nats: NatsClient) -> Self {
        Self { nats }
    }

    /// Subscribe to `ETL.data.<stream_id>.<table_name>` and return an
    /// async stream of deserialized [`StreamEnvelope`] messages.
    pub async fn listen(
        &self,
        stream_id: i32,
        table_name: &str,
    ) -> Result<impl futures::Stream<Item = Result<StreamEnvelope>> + '_> {
        let subject = StreamEnvelope::subject(stream_id, table_name);

        tracing::info!(subject = %subject, "Listening on NATS subject");

        let subscriber = self.nats.subscribe(&subject).await?;

        let stream = subscriber.map(move |msg| {
            let envelope = StreamEnvelope::from_bytes(&msg.payload)?;

            tracing::debug!(
                message_id = %envelope.id,
                records = envelope.data.len(),
                "Received stream envelope"
            );

            Ok(envelope)
        });

        Ok(stream)
    }
}
