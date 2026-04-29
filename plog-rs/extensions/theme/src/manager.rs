//! Theme Manager

use std::collections::HashMap;
use std::path::PathBuf;

use super::{ThemeInfo, ThemeError, ThemeStatus, MANIFEST_FILENAME, load_manifest};

pub struct ThemeManager {
    themes: HashMap<String, ThemeInfo>,
    active_theme: Option<String>,
    themes_dir: PathBuf,
}

impl ThemeManager {
    pub fn new(themes_dir: impl Into<PathBuf>) -> Self {
        Self {
            themes: HashMap::new(),
            active_theme: None,
            themes_dir: themes_dir.into(),
        }
    }

    pub fn discover(&mut self) -> Result<Vec<ThemeInfo>, ThemeError> {
        let mut discovered = Vec::new();
        if !self.themes_dir.exists() {
            return Ok(discovered);
        }

        let entries = std::fs::read_dir(&self.themes_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join(MANIFEST_FILENAME);
                if manifest_path.exists() {
                    if let Ok(manifest) = load_manifest(&manifest_path) {
                        let theme_id = manifest.id.clone();
                        let theme_info = ThemeInfo::from_manifest(manifest, path.to_str().unwrap_or(""));
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

    pub fn activate(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        if !self.themes.contains_key(theme_id) {
            return Err(ThemeError::NotFound(theme_id.to_string()));
        }
        self.active_theme = Some(theme_id.to_string());
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.status = ThemeStatus::Active;
            theme.activated_at = Some(chrono::Utc::now());
        }
        Ok(())
    }

    pub fn deactivate(&mut self, theme_id: &str) -> Result<(), ThemeError> {
        if self.active_theme.as_deref() == Some(theme_id) {
            return Err(ThemeError::IsActive(theme_id.to_string()));
        }
        if let Some(theme) = self.themes.get_mut(theme_id) {
            theme.status = ThemeStatus::Installed;
        }
        Ok(())
    }

    pub fn get_theme(&self, id: &str) -> Option<&ThemeInfo> {
        self.themes.get(id)
    }

    pub fn get_active(&self) -> Option<&ThemeInfo> {
        self.active_theme.as_ref().and_then(|id| self.themes.get(id))
    }

    pub fn get_all(&self) -> Vec<&ThemeInfo> {
        self.themes.values().collect()
    }

    pub fn count(&self) -> usize {
        self.themes.len()
    }
}
