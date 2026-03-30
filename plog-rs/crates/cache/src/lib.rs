//! Plog CMS Cache Module
//! 
//! 缓存模块 - 支持内存缓存

pub mod memory;
pub mod traits;

pub use memory::MemoryCache;
pub use traits::{Cache, CacheError, CacheResult};
