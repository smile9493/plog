//! 插件管理器

use std::collections::HashMap;

use plog_core::{PlogError, PlogResult};

use crate::types::*;

/// 插件管理器
pub struct PluginManager {
    plugins: HashMap<String, PluginInfo>,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// 加载插件
    pub fn load_plugin(&mut self, name: &str, info: PluginInfo) -> PlogResult<()> {
        if self.plugins.contains_key(name) {
            return Err(PlogError::ConflictError(format!(
                "Plugin {} already loaded",
                name
            )));
        }
        self.plugins.insert(name.to_string(), info);
        Ok(())
    }

    /// 卸载插件
    pub fn unload_plugin(&mut self, name: &str) -> PlogResult<()> {
        self.plugins
            .remove(name)
            .ok_or_else(|| PlogError::NotFoundError(format!("Plugin {} not found", name)))?;
        Ok(())
    }

    /// 获取插件信息
    pub fn get_plugin(&self, name: &str) -> Option<&PluginInfo> {
        self.plugins.get(name)
    }

    /// 获取所有插件
    pub fn get_all_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    /// 检查插件是否存在
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }
}
