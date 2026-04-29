//! 内存缓存实现
//!
//! 使用 DashMap 实现高并发无锁缓存
//!
//! Performance Analysis:
//! - DashMap 分片数 = CPU 核心数 * 4，默认值适合多核场景
//! - 读操作: O(1) 无锁访问
//! - 写操作: 仅锁定单个分片 (shard-level locking)
//! - 内存开销: 每分片一个 RwLock + HashMap 元数据
//!
//! Trade-off Analysis:
//! - P3 (performance): 使用 String 存储缓存值而非 Arc<str>
//! - 原因: 缓存值通常为 JSON 序列化字符串，生命周期短
//! - 如果缓存大量重复字符串，可考虑 Arc<str> 或 interning
//! - Decision: 保持 String，待 profiling 验证内存开销

use std::time::{Duration, Instant};

use dashmap::DashMap;

use crate::traits::{Cache, CacheResult};

/// 缓存项
#[derive(Clone)]
struct CacheItem {
    value: String,
    expires_at: Option<Instant>,
}

/// 内存缓存 (高并发无锁实现)
///
/// 使用 DashMap 替代 RwLock<HashMap>，提供更好的并发性能：
/// - 读操作无阻塞
/// - 写操作仅锁定单个分片
/// - 分片数量 = CPU 核心数 * 4
pub struct MemoryCache {
    items: DashMap<String, CacheItem>,
    default_ttl: Duration,
}

impl MemoryCache {
    /// 创建新的内存缓存
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            items: DashMap::new(),
            default_ttl,
        }
    }
    
    /// 创建带预估容量的缓存
    ///
    /// P3: 预分配容量减少首次插入时的 rehash
    pub fn with_capacity(default_ttl: Duration, capacity: usize) -> Self {
        Self {
            items: DashMap::with_capacity(capacity),
            default_ttl,
        }
    }

    /// 创建带默认 TTL 的缓存 (1 小时)
    pub fn with_default_ttl() -> Self {
        Self::new(Duration::from_secs(3600))
    }

    /// 清理过期项
    fn cleanup(&self) {
        let now = Instant::now();
        self.items.retain(|_, item| item.expires_at.map_or(true, |exp| exp > now));
    }

    fn is_alive(item: &CacheItem) -> bool {
        item.expires_at.map_or(true, |expires_at| Instant::now() < expires_at)
    }
}

impl Cache for MemoryCache {
    fn get(&self, key: &str) -> CacheResult<Option<String>> {
        match self.items.get(key) {
            Some(item) if Self::is_alive(&item) => Ok(Some(item.value.clone())),
            Some(_) => Ok(None),
            None => Ok(None),
        }
    }

    fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()> {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let expires_at = if ttl.is_zero() {
            None
        } else {
            Some(Instant::now() + ttl)
        };

        let item = CacheItem {
            value: value.to_string(),
            expires_at,
        };

        self.items.insert(key.to_string(), item);

        if self.items.len() > 1000 {
            self.cleanup();
        }

        Ok(())
    }

    fn delete(&self, key: &str) -> CacheResult<()> {
        self.items.remove(key);
        Ok(())
    }

    fn exists(&self, key: &str) -> CacheResult<bool> {
        match self.items.get(key) {
            Some(item) => Ok(Self::is_alive(&item)),
            None => Ok(false),
        }
    }

    fn clear(&self) -> CacheResult<()> {
        self.items.clear();
        Ok(())
    }

    fn ttl(&self, key: &str) -> CacheResult<Option<Duration>> {
        match self.items.get(key) {
            Some(item) if Self::is_alive(&item) => {
                Ok(item.expires_at.map(|expires_at| expires_at.saturating_duration_since(Instant::now())))
            }
            Some(_) => Ok(None),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_cache_set_get() {
        let cache = MemoryCache::with_default_ttl();

        cache.set("key1", "value1", None).unwrap();
        let value = cache.get("key1").unwrap();

        assert_eq!(value, Some("value1".to_string()));
    }

    #[test]
    fn test_memory_cache_delete() {
        let cache = MemoryCache::with_default_ttl();

        cache.set("key1", "value1", None).unwrap();
        cache.delete("key1").unwrap();

        let value = cache.get("key1").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_memory_cache_exists() {
        let cache = MemoryCache::with_default_ttl();

        assert!(!cache.exists("key1").unwrap());

        cache.set("key1", "value1", None).unwrap();
        assert!(cache.exists("key1").unwrap());
    }

    #[test]
    fn test_memory_cache_clear() {
        let cache = MemoryCache::with_default_ttl();

        cache.set("key1", "value1", None).unwrap();
        cache.set("key2", "value2", None).unwrap();

        cache.clear().unwrap();

        assert!(!cache.exists("key1").unwrap());
        assert!(!cache.exists("key2").unwrap());
    }
}
