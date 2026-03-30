//! 缓存 traits

use std::time::Duration;

/// 缓存错误
#[derive(Debug)]
pub enum CacheError {
    /// 键不存在
    NotFound,
    /// 序列化错误
    SerializationError(String),
    /// 连接错误
    ConnectionError(String),
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Key not found"),
            Self::SerializationError(e) => write!(f, "Serialization error: {}", e),
            Self::ConnectionError(e) => write!(f, "Connection error: {}", e),
            Self::Other(e) => write!(f, "Cache error: {}", e),
        }
    }
}

/// 缓存结果类型
pub type CacheResult<T> = Result<T, CacheError>;

/// 缓存 trait
pub trait Cache: Send + Sync {
    /// 获取缓存
    fn get(&self, key: &str) -> CacheResult<Option<String>>;
    
    /// 设置缓存
    fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()>;
    
    /// 删除缓存
    fn delete(&self, key: &str) -> CacheResult<()>;
    
    /// 检查是否存在
    fn exists(&self, key: &str) -> CacheResult<bool>;
    
    /// 清空缓存
    fn clear(&self) -> CacheResult<()>;
    
    /// 获取 TTL
    fn ttl(&self, key: &str) -> CacheResult<Option<Duration>>;
}

/// 异步缓存 trait
#[async_trait::async_trait]
pub trait AsyncCache: Send + Sync {
    /// 获取缓存
    async fn get(&self, key: &str) -> CacheResult<Option<String>>;
    
    /// 设置缓存
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()>;
    
    /// 删除缓存
    async fn delete(&self, key: &str) -> CacheResult<()>;
    
    /// 检查是否存在
    async fn exists(&self, key: &str) -> CacheResult<bool>;
    
    /// 清空缓存
    async fn clear(&self) -> CacheResult<()>;
    
    /// 获取 TTL
    async fn ttl(&self, key: &str) -> CacheResult<Option<Duration>>;
}
