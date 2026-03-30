//! 插件类型定义
//!
//! 基于 contracts 模块的插件 manifest 规范

use serde::{Deserialize, Serialize};
use std::path::Path;

pub use plog_contracts::plugin::{
    AdminAssets, EventSubscription, JobRegistration, MenuRegistration, PageRegistration,
    PluginCapability, PluginManifest,
};

/// 插件状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    /// 未安装
    NotInstalled,
    /// 已安装未启用
    Installed,
    /// 已启用
    Active,
    /// 已禁用
    Disabled,
    /// 错误状态
    Error,
}

/// 插件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// manifest
    pub manifest: PluginManifest,

    /// 状态
    pub status: PluginStatus,

    /// 插件目录
    pub path: String,

    /// 安装时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// 启用时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl PluginInfo {
    /// 从 manifest 创建
    pub fn from_manifest(manifest: PluginManifest, path: &str) -> Self {
        Self {
            manifest,
            status: PluginStatus::Installed,
            path: path.to_string(),
            installed_at: Some(chrono::Utc::now()),
            activated_at: None,
            error: None,
        }
    }

    /// 检查是否有指定能力
    pub fn has_capability(&self, capability: &PluginCapability) -> bool {
        self.manifest
            .capabilities
            .iter()
            .any(|c| std::mem::discriminant(c) == std::mem::discriminant(capability))
    }

    /// 获取菜单
    pub fn get_menus(&self) -> &[MenuRegistration] {
        &self.manifest.menus
    }

    /// 获取页面
    pub fn get_pages(&self) -> &[PageRegistration] {
        &self.manifest.pages
    }

    /// 获取任务
    pub fn get_jobs(&self) -> &[JobRegistration] {
        &self.manifest.jobs
    }

    /// 获取事件订阅
    pub fn get_events(&self) -> &[EventSubscription] {
        &self.manifest.events
    }
}

/// 插件 manifest 文件名
pub const MANIFEST_FILENAME: &str = "plugin.toml";

/// 从文件加载 manifest
pub fn load_manifest_from_file(path: &Path) -> Result<PluginManifest, PluginError> {
    let content = std::fs::read_to_string(path).map_err(|e| PluginError::IoError(e.to_string()))?;

    toml::from_str(&content)
        .map_err(|e| PluginError::ParseError(format!("Failed to parse manifest: {}", e)))
}

/// 从目录加载 manifest
pub fn load_manifest_from_dir(dir: &Path) -> Result<PluginManifest, PluginError> {
    let manifest_path = dir.join(MANIFEST_FILENAME);
    load_manifest_from_file(&manifest_path)
}

/// 插件错误
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    IoError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin already exists: {0}")]
    AlreadyExists(String),

    #[error("Plugin is active: {0}")]
    IsActive(String),

    #[error("Dependency missing: {0}")]
    DependencyMissing(String),

    #[error("Version mismatch: {0}")]
    VersionMismatch(String),
}

/// 插件列表响应
#[derive(Debug, Clone, Serialize)]
pub struct PluginListResponse {
    pub items: Vec<PluginInfo>,
    pub total: usize,
}

/// 插件操作响应
#[derive(Debug, Clone, Serialize)]
pub struct PluginOperationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<PluginInfo>,
}
