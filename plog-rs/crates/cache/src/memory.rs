//! 内存缓存实现

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use crate::traits::{Cache, CacheResult};

/// 缓存项
#[derive(Clone)]
struct CacheItem {
    value: String,
    expires_at: Option<Instant>,
}

/// 内存缓存
pub struct MemoryCache {
    items: RwLock<HashMap<String, CacheItem>>,
    default_ttl: Duration,
}

impl MemoryCache {
    /// 创建新的内存缓存
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            items: RwLock::new(HashMap::new()),
            default_ttl,
        }
    }

    /// 创建带默认 TTL 的缓存 (1 小时)
    pub fn with_default_ttl() -> Self {
        Self::new(Duration::from_secs(3600))
    }

    /// 清理过期项
    fn cleanup(&self) {
        let mut items = self.items.write().unwrap();
        let now = Instant::now();
        items.retain(|_, item| item.expires_at.map_or(true, |exp| exp > now));
    }
}

impl Cache for MemoryCache {
    fn get(&self, key: &str) -> CacheResult<Option<String>> {
        let items = self.items.read().unwrap();

        match items.get(key) {
            Some(item) => {
                if let Some(expires_at) = item.expires_at {
                    if Instant::now() > expires_at {
                        return Ok(None);
                    }
                }
                Ok(Some(item.value.clone()))
            }
            None => Ok(None),
        }
    }

    fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()> {
        let ttl = ttl.unwrap_or(self.default_ttl);
        let expires_at = if ttl.as_secs() > 0 {
            Some(Instant::now() + ttl)
        } else {
            None
        };

        let item = CacheItem {
            value: value.to_string(),
            expires_at,
        };

        let mut items = self.items.write().unwrap();
        items.insert(key.to_string(), item);

        if items.len() > 1000 {
            drop(items);
            self.cleanup();
        }

        Ok(())
    }

    fn delete(&self, key: &str) -> CacheResult<()> {
        let mut items = self.items.write().unwrap();
        items.remove(key);
        Ok(())
    }

    fn exists(&self, key: &str) -> CacheResult<bool> {
        let items = self.items.read().unwrap();

        match items.get(key) {
            Some(item) => {
                if let Some(expires_at) = item.expires_at {
                    Ok(Instant::now() < expires_at)
                } else {
                    Ok(true)
                }
            }
            None => Ok(false),
        }
    }

    fn clear(&self) -> CacheResult<()> {
        let mut items = self.items.write().unwrap();
        items.clear();
        Ok(())
    }

    fn ttl(&self, key: &str) -> CacheResult<Option<Duration>> {
        let items = self.items.read().unwrap();

        match items.get(key) {
            Some(item) => {
                if let Some(expires_at) = item.expires_at {
                    let now = Instant::now();
                    if now < expires_at {
                        Ok(Some(expires_at - now))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
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

    #[test]
    fn test_memory_cache_expiration() {
        let cache = MemoryCache::new(Duration::from_secs(1));

        cache
            .set("key1", "value1", Some(Duration::from_millis(100)))
            .unwrap();

        assert!(cache.exists("key1").unwrap());

        std::thread::sleep(Duration::from_millis(150));

        assert!(!cache.exists("key1").unwrap());
    }
}
