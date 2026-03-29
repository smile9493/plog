//! 主题管理器

use std::collections::HashMap;

use plog_core::{PlogError, PlogResult};

use crate::types::*;

/// 主题管理器
pub struct ThemeManager {
    themes: HashMap<String, ThemeInfo>,
    active_theme: Option<String>,
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            active_theme: None,
        }
    }

    /// 加载主题
    pub fn load_theme(&mut self, name: &str, info: ThemeInfo) -> PlogResult<()> {
        if self.themes.contains_key(name) {
            return Err(PlogError::ConflictError(format!(
                "Theme {} already loaded",
                name
            )));
        }
        self.themes.insert(name.to_string(), info);
        Ok(())
    }

    /// 卸载主题
    pub fn unload_theme(&mut self, name: &str) -> PlogResult<()> {
        if self.active_theme.as_ref() == Some(&name.to_string()) {
            return Err(PlogError::ConflictError(
                "Cannot unload active theme".into(),
            ));
        }
        self.themes
            .remove(name)
            .ok_or_else(|| PlogError::NotFoundError(format!("Theme {} not found", name)))?;
        Ok(())
    }

    /// 激活主题
    pub fn activate_theme(&mut self, name: &str) -> PlogResult<()> {
        if !self.themes.contains_key(name) {
            return Err(PlogError::NotFoundError(format!(
                "Theme {} not found",
                name
            )));
        }
        self.active_theme = Some(name.to_string());
        Ok(())
    }

    /// 获取当前激活的主题
    pub fn get_active_theme(&self) -> Option<&ThemeInfo> {
        self.active_theme
            .as_ref()
            .and_then(|name| self.themes.get(name))
    }

    /// 获取主题信息
    pub fn get_theme(&self, name: &str) -> Option<&ThemeInfo> {
        self.themes.get(name)
    }

    /// 获取所有主题
    pub fn get_all_themes(&self) -> Vec<&ThemeInfo> {
        self.themes.values().collect()
    }
}
