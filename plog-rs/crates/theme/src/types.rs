//! 主题类型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 主题信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub engine: ThemeEngine,
    pub templates: HashMap<String, String>,
    pub assets: ThemeAssets,
    pub supports: Vec<String>,
}

/// 主题引擎
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeEngine {
    Blade,
    Twig,
    Handlebars,
    Custom(String),
}

/// 主题资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeAssets {
    pub css: Vec<String>,
    pub js: Vec<String>,
    pub images: Vec<String>,
}
