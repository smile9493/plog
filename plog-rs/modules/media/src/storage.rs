//! Storage Driver

use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait StorageDriver: Send + Sync {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), String>;
    async fn load(&self, path: &str) -> Result<Vec<u8>, String>;
    async fn delete(&self, path: &str) -> Result<(), String>;
    fn url(&self, path: &str) -> String;
}

pub struct LocalStorage {
    base_path: PathBuf,
    base_url: String,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf, base_url: String) -> Self {
        Self { base_path, base_url }
    }
}

#[async_trait]
impl StorageDriver for LocalStorage {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), String> {
        let full_path = self.base_path.join(path);
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
        }
        tokio::fs::write(&full_path, data).await.map_err(|e| e.to_string())
    }

    async fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        let full_path = self.base_path.join(path);
        tokio::fs::read(&full_path).await.map_err(|e| e.to_string())
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        let full_path = self.base_path.join(path);
        tokio::fs::remove_file(&full_path).await.map_err(|e| e.to_string())
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }
}
