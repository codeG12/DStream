use serde_json::Value;

/// HTTP request type for client traits
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Value>,
}

/// HTTP response type for client traits
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Value,
}

/// Trait for HTTP client capabilities
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync {
    async fn request(&self, req: HttpRequest) -> anyhow::Result<HttpResponse>;
}
