use crate::core::errors::{ConfigError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for a Tap (source connector)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapConfig {
    /// Unique identifier for the tap
    pub name: String,

    /// Type of the tap (e.g., "rest-api", "database", "file")
    #[serde(rename = "type")]
    pub tap_type: String,

    /// Connection/endpoint configuration
    pub connection: ConnectionConfig,

    /// Authentication configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,

    /// Streams to extract (if not specified, all streams are extracted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<String>>,

    /// Additional tap-specific properties
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

/// Configuration for a Target (destination connector)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    /// Unique identifier for the target
    pub name: String,

    /// Type of the target (e.g., "postgres", "file", "s3")
    #[serde(rename = "type")]
    pub target_type: String,

    /// Connection/endpoint configuration
    pub connection: ConnectionConfig,

    /// Authentication configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,

    /// Batch size for writes
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Additional target-specific properties
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectionConfig {
    /// URL-based connection (for HTTP APIs, databases with connection strings)
    Url { url: String },

    /// Host/port-based connection
    HostPort {
        host: String,
        port: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        database: Option<String>,
    },

    /// File path-based connection
    FilePath { path: String },

    /// Custom connection properties
    Custom(HashMap<String, Value>),
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    /// No authentication
    None,

    /// API key authentication
    ApiKey {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        header: Option<String>,
    },

    /// Bearer token authentication
    Bearer { token: String },

    /// Basic authentication
    Basic { username: String, password: String },

    /// OAuth2 authentication
    OAuth2 {
        client_id: String,
        client_secret: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        refresh_token: Option<String>,
    },

    /// Custom authentication
    Custom(HashMap<String, Value>),
}

impl TapConfig {
    /// Load tap configuration from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().display().to_string();
        let contents = fs::read_to_string(&path).map_err(|e| ConfigError::LoadFailed {
            path: path_str.clone(),
            reason: e.to_string(),
        })?;

        serde_json::from_str(&contents).map_err(|e| {
            ConfigError::LoadFailed {
                path: path_str,
                reason: e.to_string(),
            }
            .into()
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(ConfigError::MissingField("name".to_string()).into());
        }

        if self.tap_type.is_empty() {
            return Err(ConfigError::MissingField("type".to_string()).into());
        }

        Ok(())
    }

    /// Get a property value by key
    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}

impl TargetConfig {
    /// Load target configuration from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().display().to_string();
        let contents = fs::read_to_string(&path).map_err(|e| ConfigError::LoadFailed {
            path: path_str.clone(),
            reason: e.to_string(),
        })?;

        serde_json::from_str(&contents).map_err(|e| {
            ConfigError::LoadFailed {
                path: path_str,
                reason: e.to_string(),
            }
            .into()
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(ConfigError::MissingField("name".to_string()).into());
        }

        if self.target_type.is_empty() {
            return Err(ConfigError::MissingField("type".to_string()).into());
        }

        if self.batch_size == 0 {
            return Err(ConfigError::InvalidValue {
                field: "batch_size".to_string(),
                reason: "must be greater than 0".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Get a property value by key
    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}

fn default_batch_size() -> usize {
    1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tap_config_validation() {
        let config = TapConfig {
            name: "test-tap".to_string(),
            tap_type: "rest-api".to_string(),
            connection: ConnectionConfig::Url {
                url: "https://api.example.com".to_string(),
            },
            auth: None,
            streams: None,
            properties: HashMap::new(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_target_config_default_batch_size() {
        let json = r#"{
            "name": "test-target",
            "type": "file",
            "connection": {
                "path": "/tmp/output"
            }
        }"#;

        let config: TargetConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.batch_size, 1000);
    }

    #[test]
    fn test_auth_config_serialization() {
        let auth = AuthConfig::ApiKey {
            key: "secret-key".to_string(),
            header: Some("X-API-Key".to_string()),
        };

        let json = serde_json::to_string(&auth).unwrap();
        assert!(json.contains("api_key"));
        assert!(json.contains("secret-key"));
    }

    #[test]
    fn test_connection_config_variants() {
        let url_conn = ConnectionConfig::Url {
            url: "postgres://localhost/db".to_string(),
        };
        assert!(serde_json::to_string(&url_conn).is_ok());

        let host_conn = ConnectionConfig::HostPort {
            host: "localhost".to_string(),
            port: 5432,
            database: Some("mydb".to_string()),
        };
        assert!(serde_json::to_string(&host_conn).is_ok());
    }
}
