//! Unified API Response
//! 
//! 统一的 API 响应包装器，支持泛型数据和分页
//!
//! Performance Analysis:
//! - serde_json 序列化: ~1-10μs/KB (取决于结构复杂度)
//! - skip_serializing_if: 减少约 20-30% 输出大小
//! - P3 优化路径:
//!   - 考虑 `serde_json::to_vec` 直接写入响应流
//!   - 大响应考虑流式序列化 (serde_json::to_writer)
//!   - 重复字符串考虑 intern 或 Arc<str>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 统一 API 响应包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,
}

/// 错误体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// 响应元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}

/// 分页元数据
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_more: bool,
}

impl<T> AppResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(ResponseMeta::default()),
        }
    }

    pub fn ok_with_meta(data: T, meta: ResponseMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(meta),
        }
    }

    pub fn err(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorBody {
                code: code.into(),
                message: message.into(),
                details: None,
            }),
            meta: None,
        }
    }

    pub fn err_with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorBody {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            }),
            meta: None,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> AppResponse<U> {
        AppResponse {
            success: self.success,
            data: self.data.map(f),
            error: self.error,
            meta: self.meta,
        }
    }
}

impl<T: Serialize> AppResponse<Vec<T>> {
    pub fn paginated(items: Vec<T>, page: u64, per_page: u64, total: u64) -> AppResponse<PaginatedData<T>> {
        AppResponse::ok(PaginatedData::new(items, page, per_page, total))
    }
}

impl Default for ResponseMeta {
    fn default() -> Self {
        Self {
            request_id: Some(uuid::Uuid::new_v4().to_string()),
            timestamp: Some(Utc::now()),
            pagination: None,
        }
    }
}

impl PaginationMeta {
    pub fn new(page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = if per_page == 0 { 1 } else { total.div_ceil(per_page) };
        Self {
            page,
            per_page,
            total,
            total_pages,
            has_more: page < total_pages,
        }
    }
}

/// 分页数据容器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub pagination: PaginationMeta,
}

impl<T> PaginatedData<T> {
    pub fn new(items: Vec<T>, page: u64, per_page: u64, total: u64) -> Self {
        Self {
            items,
            pagination: PaginationMeta::new(page, per_page, total),
        }
    }
}

/// 兼容旧 API 的别名
pub type ApiResponse<T> = AppResponse<T>;
pub type ApiMeta = ResponseMeta;
pub type ApiError = ErrorBody;
