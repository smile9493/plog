//! Plog CMS Shared
//! 
//! 共享类型和接口定义，所有模块都依赖于此包

// CI 强制 Lints - 库级别保护
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod contracts;
pub mod traits;
pub mod types;
pub mod error;

pub use contracts::*;
pub use traits::*;
pub use types::*;
pub use error::{AppError, AppResult};

#[macro_use]
pub mod macros;
