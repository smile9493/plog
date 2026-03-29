//! 存储驱动

use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// 存储错误
#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    NotFound(String),
    PermissionDenied(String),
    Other(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::NotFound(p) => write!(f, "File not found: {}", p),
            Self::PermissionDenied(p) => write!(f, "Permission denied: {}", p),
            Self::Other(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// 存储驱动 trait
pub trait StorageDriver {
    /// 保存文件
    fn save<'a>(&'a self, path: &'a str, data: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, StorageError>> + Send + 'a>>;
    
    /// 删除文件
    fn delete<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), StorageError>> + Send + 'a>>;
    
    /// 获取文件 URL
    fn url(&self, path: &str) -> String;
    
    /// 文件是否存在
    fn exists<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>>;
    
    /// 读取文件
    fn read<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>, StorageError>> + Send + 'a>>;
}

/// 本地存储驱动
pub struct LocalStorage {
    /// 存储根目录
    root: PathBuf,
    
    /// 基础 URL
    base_url: String,
}

impl LocalStorage {
    /// 创建新的本地存储
    pub fn new(root: impl Into<PathBuf>, base_url: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            base_url: base_url.into(),
        }
    }
    
    /// 获取完整路径
    fn full_path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }
    
    /// 确保目录存在
    async fn ensure_dir(&self, path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                async_fs::create_dir_all(parent).await?;
            }
        }
        Ok(())
    }
}

impl StorageDriver for LocalStorage {
    fn save<'a>(&'a self, path: &'a str, data: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, StorageError>> + Send + 'a>> {
        Box::pin(async move {
            let full_path = self.full_path(path);
            self.ensure_dir(&full_path).await?;
            async_fs::write(&full_path, data).await?;
            Ok(path.to_string())
        })
    }
    
    fn delete<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), StorageError>> + Send + 'a>> {
        Box::pin(async move {
            let full_path = self.full_path(path);
            if full_path.exists() {
                async_fs::remove_file(&full_path).await?;
            }
            Ok(())
        })
    }
    
    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }
    
    fn exists<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        Box::pin(async move {
            self.full_path(path).exists()
        })
    }
    
    fn read<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>, StorageError>> + Send + 'a>> {
        Box::pin(async move {
            let full_path = self.full_path(path);
            if !full_path.exists() {
                return Err(StorageError::NotFound(path.to_string()));
            }
            Ok(async_fs::read(&full_path).await?)
        })
    }
}
