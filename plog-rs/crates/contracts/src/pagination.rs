//! 分页/排序/筛选协议定义

use serde::{Deserialize, Serialize};

/// 分页参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// 页码 (默认 1)
    #[serde(default = "default_page")]
    pub page: u64,

    /// 每页数量 (默认 20)
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl PaginationParams {
    /// 限制每页最大数量
    pub fn normalize(&mut self) {
        if self.page < 1 {
            self.page = 1;
        }
        if self.per_page < 1 {
            self.per_page = 20;
        }
        if self.per_page > 100 {
            self.per_page = 100;
        }
    }

    /// 计算偏移量
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.per_page
    }

    /// 计算总页数
    pub fn total_pages(&self, total: u64) -> u64 {
        if total == 0 {
            return 1;
        }
        (total + self.per_page - 1) / self.per_page
    }
}

/// 排序方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Desc
    }
}

/// 排序参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortParams {
    /// 排序字段
    pub field: String,

    /// 排序方向
    #[serde(default)]
    pub order: SortOrder,
}

/// 筛选操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    /// 等于
    Eq,
    /// 不等于
    Ne,
    /// 大于
    Gt,
    /// 大于等于
    Gte,
    /// 小于
    Lt,
    /// 小于等于
    Lte,
    /// 包含
    Contains,
    /// 以...开头
    StartsWith,
    /// 以...结尾
    EndsWith,
    /// 在列表中
    In,
    /// 不在列表中
    NotIn,
    /// 为空
    IsNull,
    /// 不为空
    IsNotNull,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 字段名
    pub field: String,

    /// 操作符
    pub operator: FilterOperator,

    /// 值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// 查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryParams {
    /// 分页
    #[serde(flatten)]
    pub pagination: PaginationParams,

    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortParams>,

    /// 筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FilterCondition>>,

    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl QueryParams {
    /// 标准化参数
    pub fn normalize(&mut self) {
        self.pagination.normalize();
    }
}
