//! Theme Types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub layouts: Vec<LayoutDefinition>,
    #[serde(default)]
    pub slots: Vec<SlotDefinition>,
    #[serde(default)]
    pub page_templates: Vec<PageTemplate>,
    #[serde(default)]
    pub supported_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDefinition {
    pub id: String,
    pub name: String,
    pub file: String,
    #[serde(default)]
    pub default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotDefinition {
    pub id: String,
    pub name: String,
    pub position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTemplate {
    pub id: String,
    pub name: String,
    pub file: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeStatus {
    NotInstalled,
    Installed,
    Active,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub manifest: ThemeManifest,
    pub status: ThemeStatus,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ThemeInfo {
    pub fn from_manifest(manifest: ThemeManifest, path: &str) -> Self {
        Self {
            manifest,
            status: ThemeStatus::Installed,
            path: path.to_string(),
            installed_at: Some(chrono::Utc::now()),
            activated_at: None,
        }
    }
}

pub const MANIFEST_FILENAME: &str = "theme.toml";

#[derive(Debug)]
pub enum ThemeError {
    Io(String),
    Parse(String),
    NotFound(String),
    IsActive(String),
    Timeout(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Parse(e) => write!(f, "Parse error: {}", e),
            Self::NotFound(s) => write!(f, "Theme not found: {}", s),
            Self::IsActive(s) => write!(f, "Theme is active: {}", s),
            Self::Timeout(s) => write!(f, "Timeout: {}", s),
        }
    }
}

impl std::error::Error for ThemeError {}

impl From<std::io::Error> for ThemeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<toml::de::Error> for ThemeError {
    fn from(e: toml::de::Error) -> Self {
        Self::Parse(e.to_string())
    }
}

pub fn load_manifest(path: &std::path::Path) -> Result<ThemeManifest, ThemeError> {
    let content = std::fs::read_to_string(path)?;
    let manifest: ThemeManifest = toml::from_str(&content)?;
    Ok(manifest)
}
