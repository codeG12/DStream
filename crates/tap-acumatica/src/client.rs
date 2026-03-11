use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::config::AcumaticaConfig;

/// Synchronous Acumatica REST API client.
///
/// Uses `reqwest::blocking::Client` with a cookie jar so that the
/// session cookie returned by `/entity/auth/login` is automatically
/// sent with every subsequent request.
pub struct AcumaticaClient {
    config: AcumaticaConfig,
    http: Client,
    logged_in: bool,
}

impl AcumaticaClient {
    /// Create a new client from config.  Does **not** log in yet.
    pub fn new(config: AcumaticaConfig) -> Result<Self> {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        let http = Client::builder()
            .cookie_provider(jar)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            config,
            http,
            logged_in: false,
        })
    }

    // ── Authentication ──────────────────────────────────────────────────

    /// Log in to the Acumatica instance.
    /// Session cookie is stored automatically by the cookie jar.
    pub fn login(&mut self) -> Result<()> {
        let url = self.config.login_url();
        let mut body = json!({
            "name": self.config.username,
            "password": self.config.password,
        });

        if let Some(ref tenant) = self.config.tenant {
            body["tenant"] = Value::String(tenant.clone());
        }

        tracing::info!(url = %url, user = %self.config.username, "Logging in to Acumatica");

        let resp = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .with_context(|| format!("Login request failed for {}", url))?;

        let status = resp.status();
        if !status.is_success() {
            let body_text = resp.text().unwrap_or_default();
            anyhow::bail!(
                "Login failed with HTTP {} — {}",
                status.as_u16(),
                body_text
            );
        }

        self.logged_in = true;
        tracing::info!("Login successful");
        Ok(())
    }

    /// Log out (best-effort).
    pub fn logout(&mut self) {
        if !self.logged_in {
            return;
        }
        let url = self.config.logout_url();
        tracing::info!("Logging out");
        let _ = self.http.post(&url).send();
        self.logged_in = false;
    }

    // ── Entity Retrieval ────────────────────────────────────────────────

    /// Fetch a single page of an entity.
    ///
    /// Returns the JSON array of records.
    pub fn get_entity_page(
        &self,
        entity_name: &str,
        skip: usize,
        top: usize,
    ) -> Result<Vec<Value>> {
        let base = self.config.entity_base_url();
        let url = format!(
            "{}/{}?$top={}&$skip={}",
            base, entity_name, top, skip,
        );

        tracing::debug!(url = %url, "Fetching entity page");

        let resp = self
            .http
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .with_context(|| format!("GET {} failed", url))?;

        let status = resp.status();
        if !status.is_success() {
            let body_text = resp.text().unwrap_or_default();
            anyhow::bail!(
                "GET {} returned HTTP {} — {}",
                entity_name,
                status.as_u16(),
                body_text,
            );
        }

        let body: Value = resp.json().context("Failed to parse response JSON")?;

        // Acumatica returns either a JSON array directly,
        // or an object with a "value" key containing the array (OData style).
        let records = if let Some(arr) = body.as_array() {
            arr.clone()
        } else if let Some(arr) = body.get("value").and_then(|v| v.as_array()) {
            arr.clone()
        } else {
            // Single-record response — wrap in vec
            vec![body]
        };

        Ok(records)
    }

    /// Fetch **all** records for an entity, paginating with $top / $skip.
    pub fn get_all_entity_records(&self, entity_name: &str) -> Result<Vec<Value>> {
        let page_size = self.config.page_size;
        let mut all_records: Vec<Value> = Vec::new();
        let mut skip: usize = 0;

        loop {
            let page = self.get_entity_page(entity_name, skip, page_size)?;
            let count = page.len();
            tracing::info!(
                entity = entity_name,
                page_records = count,
                total_so_far = all_records.len() + count,
                "Fetched page"
            );

            if count == 0 {
                break;
            }

            all_records.extend(page);
            skip += page_size;

            // If we got fewer records than the page size, we're done.
            if count < page_size {
                break;
            }
        }

        tracing::info!(
            entity = entity_name,
            total = all_records.len(),
            "Finished fetching all records"
        );
        Ok(all_records)
    }
}

impl Drop for AcumaticaClient {
    fn drop(&mut self) {
        self.logout();
    }
}
