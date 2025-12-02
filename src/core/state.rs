use crate::core::errors::{Result, StateError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the state of a data synchronization process
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct State {
    /// Per-stream bookmarks for incremental extraction
    #[serde(default)]
    pub bookmarks: HashMap<String, Bookmark>,

    /// Global state values
    #[serde(default)]
    pub global: HashMap<String, Value>,

    /// Timestamp of last state update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
}

/// Bookmark for a specific stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    /// Replication key value (e.g., timestamp, ID)
    pub value: Value,

    /// Timestamp when this bookmark was created
    pub timestamp: DateTime<Utc>,

    /// Additional bookmark metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Manages state persistence and retrieval
pub struct StateManager {
    /// Path to the state file
    state_path: PathBuf,

    /// In-memory state cache
    state: State,

    /// Whether the state has been modified since last save
    dirty: bool,
}

impl StateManager {
    /// Create a new StateManager with the given state file path
    pub fn new<P: AsRef<Path>>(state_path: P) -> Self {
        Self {
            state_path: state_path.as_ref().to_path_buf(),
            state: State::default(),
            dirty: false,
        }
    }

    /// Load state from file, or create a new state if file doesn't exist
    pub fn load(&mut self) -> Result<()> {
        if !self.state_path.exists() {
            tracing::info!("State file does not exist, starting with empty state");
            self.state = State::default();
            return Ok(());
        }

        let path_str = self.state_path.display().to_string();
        let contents = fs::read_to_string(&self.state_path).map_err(|e| StateError::LoadFailed {
            path: path_str.clone(),
            reason: e.to_string(),
        })?;

        self.state = serde_json::from_str(&contents).map_err(|e| StateError::LoadFailed {
            path: path_str,
            reason: e.to_string(),
        })?;

        self.dirty = false;
        tracing::info!("Loaded state from {}", self.state_path.display());
        Ok(())
    }

    /// Save state to file
    pub fn save(&mut self) -> Result<()> {
        if !self.dirty {
            tracing::debug!("State unchanged, skipping save");
            return Ok(());
        }

        // Update last_updated timestamp
        self.state.last_updated = Some(Utc::now());

        let path_str = self.state_path.display().to_string();
        let contents = serde_json::to_string_pretty(&self.state).map_err(|e| {
            StateError::SaveFailed {
                path: path_str.clone(),
                reason: e.to_string(),
            }
        })?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.state_path.parent() {
            fs::create_dir_all(parent).map_err(|e| StateError::SaveFailed {
                path: path_str.clone(),
                reason: e.to_string(),
            })?;
        }

        fs::write(&self.state_path, contents).map_err(|e| StateError::SaveFailed {
            path: path_str.clone(),
            reason: e.to_string(),
        })?;

        self.dirty = false;
        tracing::info!("Saved state to {}", self.state_path.display());
        Ok(())
    }

    /// Get the current state
    pub fn get_state(&self) -> &State {
        &self.state
    }

    /// Get a bookmark for a specific stream
    pub fn get_bookmark(&self, stream: &str) -> Option<&Bookmark> {
        self.state.bookmarks.get(stream)
    }

    /// Set a bookmark for a specific stream
    pub fn set_bookmark(&mut self, stream: String, value: Value) {
        let bookmark = Bookmark {
            value,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        };
        self.state.bookmarks.insert(stream, bookmark);
        self.dirty = true;
    }

    /// Set a bookmark with metadata
    pub fn set_bookmark_with_metadata(
        &mut self,
        stream: String,
        value: Value,
        metadata: HashMap<String, Value>,
    ) {
        let bookmark = Bookmark {
            value,
            timestamp: Utc::now(),
            metadata,
        };
        self.state.bookmarks.insert(stream, bookmark);
        self.dirty = true;
    }

    /// Get a global state value
    pub fn get_global(&self, key: &str) -> Option<&Value> {
        self.state.global.get(key)
    }

    /// Set a global state value
    pub fn set_global(&mut self, key: String, value: Value) {
        self.state.global.insert(key, value);
        self.dirty = true;
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.state = State::default();
        self.dirty = true;
    }

    /// Merge another state into this one
    pub fn merge(&mut self, other: State) -> Result<()> {
        // Merge bookmarks - newer timestamps win
        for (stream, other_bookmark) in other.bookmarks {
            let should_update = match self.state.bookmarks.get(&stream) {
                Some(existing) => other_bookmark.timestamp > existing.timestamp,
                None => true,
            };

            if should_update {
                self.state.bookmarks.insert(stream, other_bookmark);
                self.dirty = true;
            }
        }

        // Merge global state - other values overwrite existing
        for (key, value) in other.global {
            self.state.global.insert(key, value);
            self.dirty = true;
        }

        Ok(())
    }

    /// Check if state has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl Drop for StateManager {
    fn drop(&mut self) {
        if self.dirty {
            tracing::warn!(
                "StateManager dropped with unsaved changes. State file: {}",
                self.state_path.display()
            );
        }
    }
}

