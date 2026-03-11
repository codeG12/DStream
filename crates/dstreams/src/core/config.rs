use crate::core::errors::{ConfigError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapConfig {
    pub name: String,

    #[serde(rename = "type")]
    pub tap_type: String,

    pub connection: ConnectionConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<String>>,

    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    pub name: String,

    #[serde(rename = "type")]
    pub target_type: String,

    pub connection: ConnectionConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,

    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectionConfig {
    Url {
        url: String,
    },

    HostPort {
        host: String,
        port: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        database: Option<String>,
    },

    FilePath {
        path: String,
    },

    Custom(HashMap<String, Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    None,

    ApiKey {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        header: Option<String>,
    },

    Bearer {
        token: String,
    },

    /// Basic authentication
    Basic {
        username: String,
        password: String,
    },

    OAuth2 {
        client_id: String,
        client_secret: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        token_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        refresh_token: Option<String>,
    },

    Custom(HashMap<String, Value>),
}

impl TapConfig {
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

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}

fn default_batch_size() -> usize {
    1000
}
