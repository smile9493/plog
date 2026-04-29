//! Pagination Types
//! 
//! 统一的分页、排序、筛选协议定义

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
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

    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.per_page
    }

    pub fn total_pages(&self, total: u64) -> u64 {
        if total == 0 {
            return 1;
        }
        total.div_ceil(self.per_page)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    #[default]
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortParams {
    pub field: String,
    #[serde(default)]
    pub order: SortOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryParams {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FilterCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl QueryParams {
    pub fn normalize(&mut self) {
        self.pagination.normalize();
    }
}
