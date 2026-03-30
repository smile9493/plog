//! 插件管理器

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::types::*;

/// 插件管理器
pub struct PluginManager {
    plugins: HashMap<String, PluginInfo>,
    plugins_dir: PathBuf,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new(plugins_dir: impl Into<PathBuf>) -> Self {
        Self {
            plugins: HashMap::new(),
            plugins_dir: plugins_dir.into(),
        }
    }

    /// 发现所有插件
    pub fn discover(&mut self) -> Result<Vec<PluginInfo>, PluginError> {
        let mut discovered = Vec::new();

        if !self.plugins_dir.exists() {
            return Ok(discovered);
        }

        let entries = std::fs::read_dir(&self.plugins_dir)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join(MANIFEST_FILENAME);
                if manifest_path.exists() {
                    if let Ok(manifest) = load_manifest_from_file(&manifest_path) {
                        let plugin_id = manifest.id.clone();
                        let plugin_info =
                            PluginInfo::from_manifest(manifest, path.to_str().unwrap_or(""));

                        if !self.plugins.contains_key(&plugin_id) {
                            self.plugins.insert(plugin_id.clone(), plugin_info.clone());
                        }

                        discovered.push(plugin_info);
                    }
                }
            }
        }

        Ok(discovered)
    }

    /// 加载插件
    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin_dir = self.plugins_dir.join(plugin_id);
        let manifest = load_manifest_from_dir(&plugin_dir)?;

        for dep in &manifest.dependencies {
            if !self.plugins.contains_key(dep) {
                return Err(PluginError::DependencyMissing(dep.clone()));
            }
        }

        let plugin_info = PluginInfo::from_manifest(manifest, plugin_dir.to_str().unwrap_or(""));
        self.plugins.insert(plugin_id.to_string(), plugin_info);
        Ok(())
    }

    /// 启用插件
    pub fn activate_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let deps = self
            .plugins
            .get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?
            .manifest
            .dependencies
            .clone();

        for dep in &deps {
            match self.plugins.get(dep) {
                Some(p) if p.status == PluginStatus::Active => {}
                _ => return Err(PluginError::DependencyMissing(dep.clone())),
            }
        }

        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        if plugin.status == PluginStatus::Active {
            return Err(PluginError::AlreadyExists(format!(
                "{} already active",
                plugin_id
            )));
        }

        plugin.status = PluginStatus::Active;
        plugin.activated_at = Some(chrono::Utc::now());
        Ok(())
    }

    /// 禁用插件
    pub fn deactivate_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let has_deps = self.plugins.iter().any(|(_, p)| {
            p.status == PluginStatus::Active
                && p.manifest.dependencies.contains(&plugin_id.to_string())
        });

        if has_deps {
            return Err(PluginError::DependencyMissing(format!(
                "Cannot deactivate: {}",
                plugin_id
            )));
        }

        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        if plugin.status != PluginStatus::Active {
            return Err(PluginError::NotFound(format!("{} not active", plugin_id)));
        }

        plugin.status = PluginStatus::Disabled;
        Ok(())
    }

    /// 卸载插件
    pub fn uninstall_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let path = self
            .plugins
            .get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?
            .path
            .clone();

        if self.plugins.get(plugin_id).unwrap().status == PluginStatus::Active {
            return Err(PluginError::IsActive(plugin_id.to_string()));
        }

        let plugin_dir = Path::new(&path);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(plugin_dir).map_err(|e| PluginError::IoError(e.to_string()))?;
        }

        self.plugins.remove(plugin_id);
        Ok(())
    }

    /// 获取插件
    pub fn get_plugin(&self, id: &str) -> Option<&PluginInfo> {
        self.plugins.get(id)
    }

    /// 获取所有插件
    pub fn get_all_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    /// 获取已启用的插件
    pub fn get_active_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins
            .values()
            .filter(|p| p.status == PluginStatus::Active)
            .collect()
    }

    /// 检查插件是否存在
    pub fn has_plugin(&self, id: &str) -> bool {
        self.plugins.contains_key(id)
    }

    /// 获取所有菜单
    pub fn get_all_menus(&self) -> Vec<MenuRegistration> {
        self.get_active_plugins()
            .iter()
            .flat_map(|p| p.get_menus().to_vec())
            .collect()
    }

    /// 获取所有页面
    pub fn get_all_pages(&self) -> Vec<PageRegistration> {
        self.get_active_plugins()
            .iter()
            .flat_map(|p| p.get_pages().to_vec())
            .collect()
    }

    /// 获取所有任务
    pub fn get_all_jobs(&self) -> Vec<JobRegistration> {
        self.get_active_plugins()
            .iter()
            .flat_map(|p| p.get_jobs().to_vec())
            .collect()
    }

    /// 获取所有事件
    pub fn get_all_events(&self) -> Vec<EventSubscription> {
        self.get_active_plugins()
            .iter()
            .flat_map(|p| p.get_events().to_vec())
            .collect()
    }

    /// 插件数量
    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}
