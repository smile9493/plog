//! API 响应格式定义

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// 统一 API 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 是否成功
    pub success: bool,

    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,

    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ApiMeta>,
}

/// API 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMeta {
    /// 请求 ID
    pub request_id: String,

    /// 时间戳
    pub timestamp: DateTime<Utc>,

    /// 分页信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}

/// 分页元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// 当前页码
    pub page: u64,

    /// 每页数量
    pub per_page: u64,

    /// 总数
    pub total: u64,

    /// 总页数
    pub total_pages: u64,

    /// 是否有更多
    pub has_more: bool,
}

/// 分页数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedData<T> {
    /// 数据列表
    pub items: Vec<T>,

    /// 分页信息
    pub pagination: PaginationMeta,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(ApiMeta {
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                pagination: None,
            }),
        }
    }

    /// 创建带分页的成功响应
    pub fn success_with_pagination(data: T, pagination: PaginationMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(ApiMeta {
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                pagination: Some(pagination),
            }),
        }
    }

    /// 创建错误响应
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: None,
            }),
            meta: Some(ApiMeta {
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                pagination: None,
            }),
        }
    }

    /// 创建带详情的错误响应
    pub fn error_with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            }),
            meta: Some(ApiMeta {
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                pagination: None,
            }),
        }
    }
}

/// 转换为 JSON
impl<T: Serialize> ApiResponse<T> {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
