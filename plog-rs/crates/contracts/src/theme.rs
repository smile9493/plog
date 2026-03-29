//! 主题 manifest 规范

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题 manifest (theme.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeManifest {
    /// 主题 ID
    pub id: String,

    /// 主题名称
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

    /// 预览图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,

    /// 布局定义
    #[serde(default)]
    pub layouts: Vec<LayoutDefinition>,

    /// 插槽定义
    #[serde(default)]
    pub slots: Vec<SlotDefinition>,

    /// 页面模板
    #[serde(default)]
    pub page_templates: Vec<PageTemplate>,

    /// 设置 Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_schema: Option<serde_json::Value>,

    /// 默认设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_defaults: Option<HashMap<String, serde_json::Value>>,

    /// 资源文件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ThemeAssets>,

    /// 支持的特性
    #[serde(default)]
    pub supported_features: Vec<ThemeFeature>,
}

/// 布局定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDefinition {
    /// 布局 ID
    pub id: String,

    /// 布局名称
    pub name: String,

    /// 模板文件
    pub template: String,

    /// 是否默认
    #[serde(default)]
    pub default: bool,
}

/// 插槽定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotDefinition {
    /// 插槽 ID
    pub id: String,

    /// 插槽名称
    pub name: String,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 默认内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// 页面模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTemplate {
    /// 模板 ID
    pub id: String,

    /// 模板名称
    pub name: String,

    /// 模板文件
    pub template: String,

    /// 适用的页面类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<String>,
}

/// 主题资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeAssets {
    /// CSS 文件
    #[serde(default)]
    pub css: Vec<String>,

    /// JS 文件
    #[serde(default)]
    pub js: Vec<String>,

    /// 图片
    #[serde(default)]
    pub images: Vec<String>,

    /// 字体
    #[serde(default)]
    pub fonts: Vec<String>,
}

/// 主题特性
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeFeature {
    /// 响应式设计
    Responsive,
    /// 暗色模式
    DarkMode,
    /// 自定义颜色
    CustomColors,
    /// 自定义字体
    CustomFonts,
    /// 侧边栏
    Sidebar,
    /// 导航菜单
    NavigationMenu,
    /// 面包屑
    Breadcrumbs,
    /// 分页
    Pagination,
    /// 评论
    Comments,
    /// 搜索
    Search,
    /// 标签云
    TagCloud,
    /// 归档
    Archive,
    /// RSS
    Rss,
}

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

    /// 安装时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// 激活时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,
}
