//! Workflow data structures for recording and replay.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub version: u32,
    pub name: String,
    pub created_at: String,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub seq: u32,
    pub command: String,
    pub args: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_hash: Option<String>,
    pub timestamp_ms: u64,
}

impl Workflow {
    fn workflows_dir() -> PathBuf {
        let dir = crate::data_dir().join("workflows");
        std::fs::create_dir_all(&dir).ok();
        dir
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::workflows_dir().join(format!("{}.json", self.name));
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }

    pub fn load(name: &str) -> std::io::Result<Self> {
        let path = Self::workflows_dir().join(format!("{name}.json"));
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(std::io::Error::other)
    }

    pub fn list() -> std::io::Result<Vec<String>> {
        let dir = Self::workflows_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut names = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    names.push(stem.to_string_lossy().to_string());
                }
            }
        }
        names.sort();
        Ok(names)
    }

    pub fn delete(name: &str) -> std::io::Result<()> {
        let path = Self::workflows_dir().join(format!("{name}.json"));
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}
