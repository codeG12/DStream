use crate::core::http::HttpResponse;
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::{Body, Client as req_client, Method};
use serde_json::Value;
use std::collections::HashMap;

use futures::future::join_all;

pub struct Client {
    session_token: Option<String>,
    refresh_token: Option<String>,
    refresh_token_url: Option<String>,
    token: Option<String>,
    header: Option<HashMap<String, String>>,
    timeout: Option<std::time::Duration>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            session_token: None,
            refresh_token: None,
            refresh_token_url: None,
            token: None,
            header: None,
            timeout: None,
        }
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_default_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.header = Some(headers);
        self
    }

    pub fn set_session_token(&mut self, token: String) {
        self.session_token = Some(token);
    }

    pub fn set_refresh_token(&mut self, refresh_token: String, refresh_url: String) {
        self.refresh_token = Some(refresh_token);
        self.refresh_token_url = Some(refresh_url);
    }
    pub async fn get(&self, url: &str, headers: HeaderMap) -> anyhow::Result<HttpResponse> {
        self.request(url, Method::GET, None, headers, self.timeout)
            .await
    }

    pub async fn async_get(&self, urls: Vec<&str>) -> anyhow::Result<Vec<HttpResponse>> {
        let futures = urls
            .into_iter()
            .map(|url| self.request(url, Method::GET, None, HeaderMap::new(), self.timeout));

        let results = join_all(futures).await;

        results.into_iter().collect()
    }

    pub async fn request(
        &self,
        url: &str,
        method: Method,
        body: Option<Body>,
        headers: HeaderMap,
        timeout: Option<core::time::Duration>,
    ) -> anyhow::Result<HttpResponse> {
        let timeout = match timeout {
            Some(t) => t,
            _ => core::time::Duration::default(),
        };

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
