//! Plugin Manager

use std::collections::HashMap;
use std::path::PathBuf;

use super::{PluginInfo, PluginError, PluginStatus, MANIFEST_FILENAME, load_manifest};

pub struct PluginManager {
    plugins: HashMap<String, PluginInfo>,
    plugins_dir: PathBuf,
}

impl PluginManager {
    pub fn new(plugins_dir: impl Into<PathBuf>) -> Self {
        Self {
            plugins: HashMap::new(),
            plugins_dir: plugins_dir.into(),
        }
    }

    pub fn discover(&mut self) -> Result<Vec<PluginInfo>, PluginError> {
        let mut discovered = Vec::new();
        if !self.plugins_dir.exists() {
            return Ok(discovered);
        }

        let entries = std::fs::read_dir(&self.plugins_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join(MANIFEST_FILENAME);
                if manifest_path.exists() {
                    if let Ok(manifest) = load_manifest(&manifest_path) {
                        let plugin_id = manifest.id.clone();
                        let plugin_info = PluginInfo::from_manifest(manifest, path.to_str().unwrap_or(""));
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

    pub fn activate(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin = self.plugins.get_mut(plugin_id).ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        plugin.status = PluginStatus::Active;
        plugin.activated_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn deactivate(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        let plugin = self.plugins.get_mut(plugin_id).ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        if plugin.status == PluginStatus::Active {
            plugin.status = PluginStatus::Disabled;
        }
        Ok(())
    }

    pub fn get_plugin(&self, id: &str) -> Option<&PluginInfo> {
        self.plugins.get(id)
    }

    pub fn get_all(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}
