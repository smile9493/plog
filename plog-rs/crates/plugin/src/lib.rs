//! Plog CMS Plugin Module
//!
//! 插件管理模块 - 基于 manifest 驱动

pub mod manager;
pub mod types;

pub use manager::PluginManager;
pub use types::*;
