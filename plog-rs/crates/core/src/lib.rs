//! Plog CMS Core Library
//! 
//! 核心库，提供基础类型和工具函数

pub mod config;
pub mod error;
pub mod types;

pub use error::{PlogError, PlogResult};
pub use types::*;
