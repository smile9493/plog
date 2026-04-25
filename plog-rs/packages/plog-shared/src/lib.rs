//! Plog CMS Shared
//! 
//! 共享类型和接口定义，所有模块都依赖于此包

pub mod contracts;
pub mod traits;
pub mod types;

pub use contracts::*;
pub use traits::*;
pub use types::*;

#[macro_use]
pub mod macros;
