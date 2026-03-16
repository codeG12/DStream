use std::path::Path;
use serde::Deserialize;
use anyhow::{Context, Result};

/// Configuration for connecting to an Acumatica instance.
#[derive(Debug, Clone, Deserialize)]
pub struct AcumaticaConfig {
    /// Base URL of the Acumatica instance, e.g. "https://mycompany.acumatica.com"
    pub instance_url: String,

    /// Login username
    pub username: String,

    /// Login password
    pub password: String,

    /// Tenant / company name (optional for single-tenant instances)
    #[serde(default)]
    pub tenant: Option<String>,

    /// REST endpoint name (default: "Default")
    #[serde(default = "default_endpoint_name")]
    pub endpoint_name: String,

    /// REST endpoint version (default: "24.200.001")
    #[serde(default = "default_endpoint_version")]
    pub endpoint_version: String,

    /// Page size for pagination (default: 100)
    #[serde(default = "default_page_size")]
    pub page_size: usize,

    /// Stream mappings
    #[serde(default)]
    pub tables: std::collections::HashMap<String, TableConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TableConfig {
    #[serde(default)]
    pub orderby_columns: Vec<String>,
    #[serde(default)]
    pub filter_conditions: String,
    #[serde(default)]
    pub key_properties: Vec<String>,
    #[serde(default)]
    pub replication_method: String,
    #[serde(default)]
    pub valid_replication_keys: String,
}

fn default_endpoint_name() -> String {
    "Default".to_string()
}

fn default_endpoint_version() -> String {
    "24.200.001".to_string()
}

fn default_page_size() -> usize {
    100
}

impl AcumaticaConfig {
    /// Load config from a JSON file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        let config: Self = serde_json::from_str(&contents)
            .with_context(|| "Failed to parse config JSON")?;
        config.validate()?;
        Ok(config)
    }

    /// Validate required fields.
    pub fn validate(&self) -> Result<()> {
        anyhow::ensure!(!self.instance_url.is_empty(), "instance_url is required");
        anyhow::ensure!(!self.username.is_empty(), "username is required");
        anyhow::ensure!(!self.password.is_empty(), "password is required");
        Ok(())
    }

    /// Build the base entity API URL.
    /// e.g. "https://instance.com/entity/Default/24.200.001"
    pub fn entity_base_url(&self) -> String {
        let base = self.instance_url.trim_end_matches('/');
        format!("{}/entity/{}/{}", base, self.endpoint_name, self.endpoint_version)
    }

    /// Build the auth login URL.
    pub fn login_url(&self) -> String {
        let base = self.instance_url.trim_end_matches('/');
        format!("{}/entity/auth/login", base)
    }

    /// Build the auth logout URL.
    pub fn logout_url(&self) -> String {
        let base = self.instance_url.trim_end_matches('/');
        format!("{}/entity/auth/logout", base)
    }
}
