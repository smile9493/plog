//! Plugin Types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    NotInstalled,
    Installed,
    Active,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub manifest: PluginManifest,
    pub status: PluginStatus,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl PluginInfo {
    pub fn from_manifest(manifest: PluginManifest, path: &str) -> Self {
        Self {
            manifest,
            status: PluginStatus::Installed,
            path: path.to_string(),
            installed_at: Some(chrono::Utc::now()),
            activated_at: None,
        }
    }
}

pub const MANIFEST_FILENAME: &str = "plugin.toml";

#[derive(Debug)]
pub enum PluginError {
    Io(String),
    Parse(String),
    NotFound(String),
    IsActive(String),
    DependencyMissing(String),
    Timeout(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Parse(e) => write!(f, "Parse error: {}", e),
            Self::NotFound(s) => write!(f, "Plugin not found: {}", s),
            Self::IsActive(s) => write!(f, "Plugin is active: {}", s),
            Self::DependencyMissing(s) => write!(f, "Dependency missing: {}", s),
            Self::Timeout(s) => write!(f, "Timeout: {}", s),
        }
    }
}

impl std::error::Error for PluginError {}

impl From<std::io::Error> for PluginError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<toml::de::Error> for PluginError {
    fn from(e: toml::de::Error) -> Self {
        Self::Parse(e.to_string())
    }
}

pub fn load_manifest(path: &std::path::Path) -> Result<PluginManifest, PluginError> {
    let content = std::fs::read_to_string(path)?;
    let manifest: PluginManifest = toml::from_str(&content)?;
    Ok(manifest)
}
