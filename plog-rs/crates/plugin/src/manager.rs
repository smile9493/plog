//! 插件管理器

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::types::*;

const DISCOVER_TIMEOUT: Duration = Duration::from_secs(2);
const IO_TIMEOUT: Duration = Duration::from_secs(2);

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

        let entries = std::fs::read_dir(&self.plugins_dir).map_err(|source| PluginError::Io {
            path: self.plugins_dir.clone(),
            source,
        })?;

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

    /// 异步发现所有插件
    pub async fn discover_async(&mut self) -> Result<Vec<PluginInfo>, PluginError> {
        tracing::debug!("Discover plugins started: dir={}", self.plugins_dir.display());
        let mut discovered = Vec::new();

        if !self.plugins_dir.exists() {
            tracing::debug!("Plugin dir does not exist, skip discovery");
            return Ok(discovered);
        }

        let mut entries = tokio::fs::read_dir(&self.plugins_dir)
            .await
            .map_err(|source| PluginError::Io {
                path: self.plugins_dir.clone(),
                source,
            })?;

        while let Some(entry) = entries.next_entry().await.map_err(|source| PluginError::Io {
            path: self.plugins_dir.clone(),
            source,
        })? {
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join(MANIFEST_FILENAME);
                if manifest_path.exists() {
                    if let Ok(manifest) = load_manifest_from_file_async(&manifest_path).await {
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

        tracing::debug!("Discover plugins completed: count={}", discovered.len());
        Ok(discovered)
    }

    /// 异步发现所有插件（带超时保护）
    pub async fn discover_async_with_timeout(&mut self) -> Result<Vec<PluginInfo>, PluginError> {
        match tokio::time::timeout(DISCOVER_TIMEOUT, self.discover_async()).await {
            Ok(result) => result,
            Err(_) => {
                tracing::warn!(
                    "Discover plugins timeout after {}s: dir={}",
                    DISCOVER_TIMEOUT.as_secs(),
                    self.plugins_dir.display()
                );
                Err(PluginError::Timeout(format!(
                    "Plugin discovery timed out after {}s",
                    DISCOVER_TIMEOUT.as_secs()
                )))
            }
        }
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

    /// 异步加载插件
    pub async fn load_plugin_async(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin_dir = self.plugins_dir.join(plugin_id);
        tracing::debug!("Load plugin started: id={}", plugin_id);
        let manifest = match tokio::time::timeout(IO_TIMEOUT, load_manifest_from_dir_async(&plugin_dir)).await {
            Ok(result) => result?,
            Err(_) => {
                tracing::warn!(
                    "Load plugin timeout after {}s: id={}, dir={}",
                    IO_TIMEOUT.as_secs(),
                    plugin_id,
                    plugin_dir.display()
                );
                return Err(PluginError::Timeout(format!(
                    "Plugin load timed out after {}s: {}",
                    IO_TIMEOUT.as_secs(),
                    plugin_id
                )));
            }
        };

        for dep in &manifest.dependencies {
            if !self.plugins.contains_key(dep) {
                return Err(PluginError::DependencyMissing(dep.clone()));
            }
        }

        let plugin_info = PluginInfo::from_manifest(manifest, plugin_dir.to_str().unwrap_or(""));
        self.plugins.insert(plugin_id.to_string(), plugin_info);
        tracing::debug!("Load plugin completed: id={}", plugin_id);
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
                && p.manifest.dependencies.iter().any(|dep| dep == plugin_id)
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
        let plugin = self
            .plugins
            .get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        if plugin.status == PluginStatus::Active {
            return Err(PluginError::IsActive(plugin_id.to_string()));
        }

        let plugin_dir = Path::new(&plugin.path);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(plugin_dir).map_err(|source| PluginError::Io {
                path: plugin_dir.to_path_buf(),
                source,
            })?;
        }

        self.plugins.remove(plugin_id);
        Ok(())
    }

    /// 异步卸载插件
    pub async fn uninstall_plugin_async(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin = self
            .plugins
            .get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;

        if plugin.status == PluginStatus::Active {
            return Err(PluginError::IsActive(plugin_id.to_string()));
        }

        let plugin_dir = Path::new(&plugin.path);
        if plugin_dir.exists() {
            let delete_fut = tokio::fs::remove_dir_all(plugin_dir);
            match tokio::time::timeout(IO_TIMEOUT, delete_fut).await {
                Ok(result) => result.map_err(|source| PluginError::Io {
                    path: plugin_dir.to_path_buf(),
                    source,
                })?,
                Err(_) => {
                    tracing::warn!(
                        "Uninstall plugin timeout after {}s: id={}, dir={}",
                        IO_TIMEOUT.as_secs(),
                        plugin_id,
                        plugin_dir.display()
                    );
                    return Err(PluginError::Timeout(format!(
                        "Plugin uninstall timed out after {}s: {}",
                        IO_TIMEOUT.as_secs(),
                        plugin_id
                    )));
                }
            }
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
