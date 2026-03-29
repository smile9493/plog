//! 插件 manifest 规范

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 插件 manifest (plugin.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// 插件 ID
    pub id: String,

    /// 插件名称
    pub name: String,

    /// 版本
    pub version: String,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// 作者主页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    /// 许可证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// 最低 Plog 版本要求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<String>,

    /// 依赖的其他插件
    #[serde(default)]
    pub dependencies: Vec<String>,

    /// 能力声明
    #[serde(default)]
    pub capabilities: Vec<PluginCapability>,

    /// 权限声明
    #[serde(default)]
    pub permissions: Vec<String>,

    /// 设置 Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_schema: Option<serde_json::Value>,

    /// 默认设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_defaults: Option<HashMap<String, serde_json::Value>>,

    /// 菜单注册
    #[serde(default)]
    pub menus: Vec<MenuRegistration>,

    /// 页面注册
    #[serde(default)]
    pub pages: Vec<PageRegistration>,

    /// 定时任务
    #[serde(default)]
    pub jobs: Vec<JobRegistration>,

    /// 事件订阅
    #[serde(default)]
    pub events: Vec<EventSubscription>,

    /// 数据库迁移
    #[serde(default)]
    pub migrations: Vec<String>,

    /// 管理后台资源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_assets: Option<AdminAssets>,
}

/// 插件能力
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginCapability {
    /// 提供 API 端点
    ApiEndpoint,
    /// 提供管理页面
    AdminPage,
    /// 提供前台组件
    FrontendWidget,
    /// 提供命令
    Command,
    /// 提供定时任务
    ScheduledJob,
    /// 提供事件处理
    EventHandler,
    /// 提供过滤器
    Filter,
    /// 提供短代码
    Shortcode,
}

/// 菜单注册
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuRegistration {
    /// 菜单 ID
    pub id: String,

    /// 菜单标题
    pub title: String,

    /// 菜单图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 父菜单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// 路由路径
    pub route: String,

    /// 排序
    #[serde(default)]
    pub order: i32,

    /// 所需权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

/// 页面注册
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRegistration {
    /// 页面 ID
    pub id: String,

    /// 页面标题
    pub title: String,

    /// 路由路径
    pub route: String,

    /// 组件路径
    pub component: String,

    /// 布局
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,

    /// 所需权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

/// 定时任务注册
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRegistration {
    /// 任务 ID
    pub id: String,

    /// 任务名称
    pub name: String,

    /// Cron 表达式
    pub schedule: String,

    /// 处理器
    pub handler: String,

    /// 是否启用
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

/// 事件订阅
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscription {
    /// 事件名称
    pub event: String,

    /// 处理器
    pub handler: String,

    /// 优先级
    #[serde(default = "default_priority")]
    pub priority: i32,
}

fn default_priority() -> i32 {
    10
}

/// 管理后台资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAssets {
    /// JS 文件
    #[serde(default)]
    pub js: Vec<String>,

    /// CSS 文件
    #[serde(default)]
    pub css: Vec<String>,
}

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
