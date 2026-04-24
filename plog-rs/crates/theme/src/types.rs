//! 主题类型定义
//!
//! 基于 contracts 模块的主题 manifest 规范

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub use plog_contracts::theme::{
    LayoutDefinition, PageTemplate, SlotDefinition, ThemeAssets as ManifestAssets, ThemeFeature,
    ThemeManifest,
};

/// 主题状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeStatus {
    /// 未安装
    NotInstalled,
    /// 已安装
    Installed,
    /// 已激活
    Active,
}

/// 主题信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    /// manifest
    pub manifest: ThemeManifest,

    /// 状态
    pub status: ThemeStatus,

    /// 主题目录
    pub path: String,

    /// 安装时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// 激活时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ThemeInfo {
    /// 从 manifest 创建
    pub fn from_manifest(manifest: ThemeManifest, path: &str) -> Self {
        Self {
            manifest,
            status: ThemeStatus::Installed,
            path: path.to_string(),
            installed_at: Some(chrono::Utc::now()),
            activated_at: None,
        }
    }

    /// 获取布局
    pub fn get_layout(&self, id: &str) -> Option<&LayoutDefinition> {
        self.manifest.layouts.iter().find(|l| l.id == id)
    }

    /// 获取默认布局
    pub fn get_default_layout(&self) -> Option<&LayoutDefinition> {
        self.manifest.layouts.iter().find(|l| l.default)
    }

    /// 获取插槽
    pub fn get_slots(&self) -> &[SlotDefinition] {
        &self.manifest.slots
    }

    /// 获取页面模板
    pub fn get_page_templates(&self) -> &[PageTemplate] {
        &self.manifest.page_templates
    }

    /// 检查是否支持特性
    pub fn supports_feature(&self, feature: &ThemeFeature) -> bool {
        self.manifest
            .supported_features
            .iter()
            .any(|f| std::mem::discriminant(f) == std::mem::discriminant(feature))
    }
}

/// 主题 manifest 文件名
pub const MANIFEST_FILENAME: &str = "theme.toml";

/// 从文件加载 manifest
pub fn load_manifest_from_file(path: &Path) -> Result<ThemeManifest, ThemeError> {
    let content = std::fs::read_to_string(path).map_err(|source| ThemeError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    toml::from_str(&content).map_err(|source| ThemeError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

/// 从文件异步加载 manifest
pub async fn load_manifest_from_file_async(path: &Path) -> Result<ThemeManifest, ThemeError> {
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|source| ThemeError::Io {
            path: path.to_path_buf(),
            source,
        })?;

    toml::from_str(&content).map_err(|source| ThemeError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

/// 从目录加载 manifest
pub fn load_manifest_from_dir(dir: &Path) -> Result<ThemeManifest, ThemeError> {
    let manifest_path = dir.join(MANIFEST_FILENAME);
    load_manifest_from_file(&manifest_path)
}

/// 从目录异步加载 manifest
pub async fn load_manifest_from_dir_async(dir: &Path) -> Result<ThemeManifest, ThemeError> {
    let manifest_path = dir.join(MANIFEST_FILENAME);
    load_manifest_from_file_async(&manifest_path).await
}

/// 主题错误
#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("IO error at `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Parse error at `{path}`: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Template error at `{path}`: {source}")]
    Template {
        path: PathBuf,
        #[source]
        source: tera::Error,
    },

    #[error("Context serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Theme not found: {0}")]
    NotFound(String),

    #[error("Theme already exists: {0}")]
    AlreadyExists(String),

    #[error("Theme is active: {0}")]
    IsActive(String),

    #[error("Operation timeout: {0}")]
    Timeout(String),
}

/// 主题列表响应
#[derive(Debug, Clone, Serialize)]
pub struct ThemeListResponse {
    pub items: Vec<ThemeInfo>,
    pub total: usize,
}

/// 主题操作响应
#[derive(Debug, Clone, Serialize)]
pub struct ThemeOperationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<ThemeInfo>,
}
