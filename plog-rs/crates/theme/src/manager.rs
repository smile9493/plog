//! 主题管理器

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::types::*;

const DISCOVER_TIMEOUT: Duration = Duration::from_secs(2);
const IO_TIMEOUT: Duration = Duration::from_secs(2);

/// 主题管理器
pub struct ThemeManager {
    themes: HashMap<String, ThemeInfo>,
    active_theme: Option<String>,
    themes_dir: PathBuf,
}

impl ThemeManager {
    /// 创建新的主题管理器
    pub fn new(themes_dir: impl Into<PathBuf>) -> Self {
        Self {
            themes: HashMap::new(),
            active_theme: None,
            themes_dir: themes_dir.into(),
        }
    }

    /// 发现所有主题
    pub fn discover(&mut self) -> Result<Vec<ThemeInfo>, ThemeError> {
        // P3: 预分配容量，典型场景 <10 个主题
        let mut discovered = Vec::with_capacity(8);

        if !self.themes_dir.exists() {
            return Ok(discovered);
        }

        let entries = std::fs::read_dir(&self.themes_dir).map_err(|source| ThemeError::Io {
            path: self.themes_dir.clone(),
            source,
        })?;

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join(MANIFEST_FILENAME);
                if manifest_path.exists() {
                    if let Ok(manifest) = load_manifest_from_file(&manifest_path) {
                        let theme_id = manifest.id.clone();
                        let theme_info =
                            ThemeInfo::from_manifest(manifest, path.to_str().unwrap_or(""));

                        if !self.themes.contains_key(&theme_id) {
                            self.themes.insert(theme_id.clone(), theme_info.clone());
                        }

                        discovered.push(theme_info);
                    }
                }
            }
        }

        Ok(discovered)
    }

    /// 异步发现所有主题
    pub async fn discover_async(&mut self) -> Result<Vec<ThemeInfo>, ThemeError> {
        tracing::debug!("Discover themes started: dir={}", self.themes_dir.display());
        // P3: 预分配容量
        let mut discovered = Vec::with_capacity(8);

        if !self.themes_dir.exists() {
            tracing::debug!("Theme dir does not exist, skip discovery");
            return Ok(discovered);
        }

        let mut entries = tokio::fs::read_dir(&self.themes_dir)
            .await
            .map_err(|source| ThemeError::Io {
                path: self.themes_dir.clone(),
                source,
            })?;

        while let Some(entry) = entries.next_entry().await.map_err(|source| ThemeError::Io {
            path: self.themes_dir.clone(),
            source,
        })? {
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join(MANIFEST_FILENAME);
                if manifest_path.exists() {
                    if let Ok(manifest) = load_manifest_from_file_async(&manifest_path).await {
                        let theme_id = manifest.id.clone();
                        let theme_info =
                            ThemeInfo::from_manifest(manifest, path.to_str().unwrap_or(""));

                        if !self.themes.contains_key(&theme_id) {
                            self.themes.insert(theme_id.clone(), theme_info.clone());
                        }

                        discovered.push(theme_info);
                    }
                }
            }
        }

        tracing::debug!("Discover themes completed: count={}", discovered.len());
        Ok(discovered)
    }

    /// 异步发现所有主题（带超时保护）
    pub async fn discover_async_with_timeout(&mut self) -> Result<Vec<ThemeInfo>, ThemeError> {
        match tokio::time::timeout(DISCOVER_TIMEOUT, self.discover_async()).await {
            Ok(result) => result,
            Err(_) => {
                tracing::warn!(
                    "Discover themes timeout after {}s: dir={}",
                    DISCOVER_TIMEOUT.as_secs(),
                    self.themes_dir.display()
                );
                Err(ThemeError::Timeout(format!(
                    "Theme discovery timed out after {}s",
                    DISCOVER_TIMEOUT.as_secs()
                )))
            }
        }
    }

    /// 加载主题
    pub fn load_theme(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        let theme_dir = self.themes_dir.join(theme_id);
        let manifest = load_manifest_from_dir(&theme_dir)?;

        let theme_info = ThemeInfo::from_manifest(manifest, theme_dir.to_str().unwrap_or(""));
        self.themes.insert(theme_id.to_string(), theme_info);
        Ok(())
    }

    /// 异步加载主题
    pub async fn load_theme_async(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        let theme_dir = self.themes_dir.join(theme_id);
        tracing::debug!("Load theme started: id={}", theme_id);
        let manifest = match tokio::time::timeout(IO_TIMEOUT, load_manifest_from_dir_async(&theme_dir)).await {
            Ok(result) => result?,
            Err(_) => {
                tracing::warn!(
                    "Load theme timeout after {}s: id={}, dir={}",
                    IO_TIMEOUT.as_secs(),
                    theme_id,
                    theme_dir.display()
                );
                return Err(ThemeError::Timeout(format!(
                    "Theme load timed out after {}s: {}",
                    IO_TIMEOUT.as_secs(),
                    theme_id
                )));
            }
        };

        let theme_info = ThemeInfo::from_manifest(manifest, theme_dir.to_str().unwrap_or(""));
        self.themes.insert(theme_id.to_string(), theme_info);
        tracing::debug!("Load theme completed: id={}", theme_id);
        Ok(())
    }

    /// 激活主题
    pub fn activate_theme(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        if !self.themes.contains_key(theme_id) {
            return Err(ThemeError::NotFound(theme_id.to_string()));
        }

        self.active_theme = Some(theme_id.to_string());

        // 更新状态
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.status = ThemeStatus::Active;
            theme.activated_at = Some(chrono::Utc::now());
        }

        Ok(())
    }

    /// 停用主题
    pub fn deactivate_theme(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        if self.active_theme.as_deref() == Some(theme_id) {
            return Err(ThemeError::IsActive(theme_id.to_string()));
        }

        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.status = ThemeStatus::Installed;
        }

        Ok(())
    }

    /// 卸载主题
    pub fn uninstall_theme(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        if self.active_theme.as_deref() == Some(theme_id) {
            return Err(ThemeError::IsActive(theme_id.to_string()));
        }

        let theme = self
            .themes
            .get(theme_id)
            .ok_or_else(|| ThemeError::NotFound(theme_id.to_string()))?;

        let theme_dir = Path::new(&theme.path);
        if theme_dir.exists() {
            std::fs::remove_dir_all(theme_dir).map_err(|source| ThemeError::Io {
                path: theme_dir.to_path_buf(),
                source,
            })?;
        }

        self.themes.remove(theme_id);
        Ok(())
    }

    /// 异步卸载主题
    pub async fn uninstall_theme_async(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        if self.active_theme.as_deref() == Some(theme_id) {
            return Err(ThemeError::IsActive(theme_id.to_string()));
        }

        let theme = self
            .themes
            .get(theme_id)
            .ok_or_else(|| ThemeError::NotFound(theme_id.to_string()))?;

        let theme_dir = Path::new(&theme.path);
        if theme_dir.exists() {
            let delete_fut = tokio::fs::remove_dir_all(theme_dir);
            match tokio::time::timeout(IO_TIMEOUT, delete_fut).await {
                Ok(result) => result.map_err(|source| ThemeError::Io {
                    path: theme_dir.to_path_buf(),
                    source,
                })?,
                Err(_) => {
                    tracing::warn!(
                        "Uninstall theme timeout after {}s: id={}, dir={}",
                        IO_TIMEOUT.as_secs(),
                        theme_id,
                        theme_dir.display()
                    );
                    return Err(ThemeError::Timeout(format!(
                        "Theme uninstall timed out after {}s: {}",
                        IO_TIMEOUT.as_secs(),
                        theme_id
                    )));
                }
            }
        }

        self.themes.remove(theme_id);
        Ok(())
    }

    /// 获取主题
    pub fn get_theme(&self, id: &str) -> Option<&ThemeInfo> {
        self.themes.get(id)
    }

    /// 获取当前激活的主题
    pub fn get_active_theme(&self) -> Option<&ThemeInfo> {
        self.active_theme
            .as_ref()
            .and_then(|id| self.themes.get(id))
    }

    /// 获取所有主题
    pub fn get_all_themes(&self) -> Vec<&ThemeInfo> {
        self.themes.values().collect()
    }

    /// 检查主题是否存在
    pub fn has_theme(&self, id: &str) -> bool {
        self.themes.contains_key(id)
    }

    /// 获取所有布局
    pub fn get_all_layouts(&self) -> Vec<LayoutDefinition> {
        self.get_active_theme()
            .map(|t| t.manifest.layouts.clone())
            .unwrap_or_default()
    }

    /// 获取所有插槽
    pub fn get_all_slots(&self) -> Vec<SlotDefinition> {
        self.get_active_theme()
            .map(|t| t.manifest.slots.clone())
            .unwrap_or_default()
    }

    /// 获取所有页面模板
    pub fn get_all_page_templates(&self) -> Vec<PageTemplate> {
        self.get_active_theme()
            .map(|t| t.manifest.page_templates.clone())
            .unwrap_or_default()
    }

    /// 主题数量
    pub fn count(&self) -> usize {
        self.themes.len()
    }
}
