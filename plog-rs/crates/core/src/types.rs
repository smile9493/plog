//! 基础类型定义

use serde::{Deserialize, Serialize};

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub per_page: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

/// 分页元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_more: bool,
}

/// 时间戳类型
pub type Timestamp = i64;

/// ID 类型
pub type Id = i32;
