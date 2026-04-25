//! Plog CMS Core
//! 
//! 微内核架构的核心模块，只包含最小依赖的基础类型
//! 
//! 这个模块是整个应用程序的最小核心，其他所有模块都依赖于此模块

pub mod error;
pub mod config;

pub use error::{CoreError, CoreResult};
pub use config::{AppConfig, DatabaseConfig, ServerConfig, AuthConfig, CorsConfig};
