#[cfg(test)]
mod tests {
    use crate::core::http::{HttpClient, HttpRequest, HttpResponse};
    use crate::core::pagination::Page;
    use crate::core::protocol::Message;
    use crate::core::traits::*;
    use anyhow::Result;
    use arrow::datatypes::SchemaRef;
    use async_trait::async_trait;
    use futures::stream::{self, BoxStream};
    use serde_json::Value;

    // ========================================================================
    // Mock HTTP Client
    // ========================================================================

    struct MockHttpClient;

    #[async_trait]
    impl HttpClient for MockHttpClient {
        async fn request(&self, _req: HttpRequest) -> Result<HttpResponse> {
            Ok(HttpResponse {
                status: 200,
                headers: vec![],
                body: Value::Null,
            })
        }
    }

    // ========================================================================
    // Mock Tap Implementations
    // ========================================================================

    struct MockHttpTap {
        client: MockHttpClient,
        authenticated: bool,
        state: Value,
    }

    impl MockHttpTap {
        fn new() -> Self {
            Self {
                client: MockHttpClient,
                authenticated: false,
                state: Value::Null,
            }
        }
    }

    impl Tap for MockHttpTap {}

    #[async_trait]
    impl Discover for MockHttpTap {
        async fn discover(&self) -> Result<()> {
            // In real implementation, would write catalog.json
            Ok(())
        }
    }

    #[async_trait]
    impl TapClient for MockHttpTap {
        fn get_client(&self) -> &dyn HttpClient {
            &self.client
        }

        async fn request(&self, req: HttpRequest) -> Result<HttpResponse> {
            self.client.request(req).await
        }
    }

    #[async_trait]
    impl TapAuth for MockHttpTap {
        async fn authenticate(&mut self, _credentials: Value) -> Result<()> {
            self.authenticated = true;
            Ok(())
        }

        async fn refresh_token(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl TapState for MockHttpTap {
        async fn get_state(&self) -> Result<Value> {
            Ok(self.state.clone())
        }

        async fn set_state(&mut self, state: Value) -> Result<()> {
            self.state = state;
            Ok(())
        }
    }

    // ========================================================================
    // Mock Stream Tap
    // ========================================================================

    struct MockStreamTap {
        page_count: usize,
        current_page: usize,
    }

    impl MockStreamTap {
        fn new() -> Self {
            Self {
                page_count: 3,
                current_page: 0,
            }
        }
    }

    impl Tap for MockStreamTap {}

    #[async_trait]
    impl TapStream for MockStreamTap {
        async fn stream(&mut self) -> BoxStream<'_, Result<Message>> {
            Box::pin(stream::empty())
        }
    }

    #[async_trait]
    impl Pagination for MockStreamTap {
        async fn next_page(&mut self) -> Result<Option<Page>> {
            if self.current_page < self.page_count {
                self.current_page += 1;
                Ok(Some(Page::new(vec![]).with_page_number(self.current_page)))
            } else {
                Ok(None)
            }
        }

        fn has_more(&self) -> bool {
            self.current_page < self.page_count
        }
    }

    #[async_trait]
    impl TapSync for MockStreamTap {
        async fn sync(&mut self) -> Result<Vec<Message>> {
            Ok(vec![])
        }
    }

    // ========================================================================
    // Mock Target Implementations
    // ========================================================================

    struct MockBatchTarget {
        client: MockHttpClient,
        in_batch: bool,
        batch_messages: Vec<Message>,
    }

    impl MockBatchTarget {
        fn new() -> Self {
            Self {
                client: MockHttpClient,
                in_batch: false,
                batch_messages: vec![],
            }
        }
    }

    impl Target for MockBatchTarget {}

    #[async_trait]
    impl TargetClient for MockBatchTarget {
        fn get_client(&self) -> &dyn HttpClient {
            &self.client
        }

        async fn request(&self, req: HttpRequest) -> Result<HttpResponse> {
            self.client.request(req).await
        }
    }

    #[async_trait]
    impl BatchSink for MockBatchTarget {
        async fn begin_batch(&mut self) -> Result<()> {
            self.in_batch = true;
            self.batch_messages.clear();
            Ok(())
        }

        async fn write_to_batch(&mut self, message: Message) -> Result<()> {
            self.batch_messages.push(message);
            Ok(())
        }

        async fn commit_batch(&mut self) -> Result<()> {
            self.in_batch = false;
            // In real implementation, would flush to destination
            Ok(())
        }

        async fn rollback_batch(&mut self) -> Result<()> {
            self.in_batch = false;
            self.batch_messages.clear();
            Ok(())
        }
    }

    #[async_trait]
    impl TargetAuth for MockBatchTarget {
        async fn authenticate(&mut self, _credentials: Value) -> Result<()> {
            Ok(())
        }

        async fn refresh_token(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl Sink for MockBatchTarget {
        async fn initialize(&mut self) -> Result<()> {
            Ok(())
        }

        async fn finalize(&mut self) -> Result<()> {
            Ok(())
        }
    }

    // ========================================================================
    // Mock Stream Target
    // ========================================================================

    struct MockStreamTarget {
        messages_written: usize,
    }

    impl MockStreamTarget {
        fn new() -> Self {
            Self {
                messages_written: 0,
            }
        }
    }

    impl Target for MockStreamTarget {}

    #[async_trait]
    impl StreamSink for MockStreamTarget {
        async fn write(&mut self, _message: Message) -> Result<()> {
            self.messages_written += 1;
            Ok(())
        }
    }

    #[async_trait]
    impl Transform for MockStreamTarget {
        async fn transform(&self, message: Message) -> Result<Message> {
            // Pass-through transformation
            Ok(message)
        }

        fn transform_schema(&self, schema: SchemaRef) -> Result<SchemaRef> {
            // Pass-through schema
            Ok(schema)
        }
    }

    #[async_trait]
    impl TargetStream for MockStreamTarget {
        async fn stream_write(&mut self, mut stream: BoxStream<'_, Result<Message>>) -> Result<()> {
            use futures::StreamExt;
            while let Some(result) = stream.next().await {
                let _message = result?;
                self.messages_written += 1;
            }
            Ok(())
        }
    }

    #[async_trait]
    impl TargetSync for MockStreamTarget {
        async fn sync_write(&mut self, messages: Vec<Message>) -> Result<()> {
            self.messages_written += messages.len();
            Ok(())
        }
    }

    // ========================================================================
    // Tests
    // ========================================================================

    #[tokio::test]
    async fn test_http_tap_composition() {
        let mut tap = MockHttpTap::new();
        
        // Test Discover
        assert!(tap.discover().await.is_ok());
        
        // Test Auth
        assert!(tap.authenticate(Value::Null).await.is_ok());
        
        // Test State
        assert!(tap.set_state(Value::String("test".to_string())).await.is_ok());
        let state = tap.get_state().await.unwrap();
        assert_eq!(state, Value::String("test".to_string()));
        
        // Test Client
        let req = HttpRequest {
            url: "http://example.com".to_string(),
            method: "GET".to_string(),
            headers: vec![],
            body: None,
        };
        assert!(tap.request(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_stream_tap_composition() {
        let mut tap = MockStreamTap::new();
        
        // Test Pagination
        assert!(tap.has_more());
        let page = tap.next_page().await.unwrap();
        assert!(page.is_some());
        
        // Test Stream
        let stream = tap.stream().await;
        drop(stream); // Drop to release mutable borrow
        
        // Test Sync
        assert!(tap.sync().await.is_ok());
    }

    #[tokio::test]
    async fn test_batch_target_composition() {
        let mut target = MockBatchTarget::new();
        
        // Test Sink lifecycle
        assert!(target.initialize().await.is_ok());
        
        // Test Auth
        assert!(target.authenticate(Value::Null).await.is_ok());
        
        // Test BatchSink
        assert!(target.begin_batch().await.is_ok());
        assert!(target.write_to_batch(Message::State(Value::Null)).await.is_ok());
        assert!(target.commit_batch().await.is_ok());
        
        assert!(target.finalize().await.is_ok());
    }

    #[tokio::test]
    async fn test_stream_target_composition() {
        let mut target = MockStreamTarget::new();
        
        // Test StreamSink
        assert!(target.write(Message::State(Value::Null)).await.is_ok());
        assert_eq!(target.messages_written, 1);
        
        // Test Transform
        let msg = Message::State(Value::Null);
        let transformed = target.transform(msg).await.unwrap();
        assert!(matches!(transformed, Message::State(_)));
        
        // Test TargetSync
        assert!(target.sync_write(vec![Message::State(Value::Null)]).await.is_ok());
        assert_eq!(target.messages_written, 2);
    }
}
