//! API Response DTOs
//! 
//! 统一的响应数据结构定义

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ApiMeta>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: Some(true),
            data: Some(data),
            error: None,
            meta: Some(ApiMeta::default()),
        }
    }

    pub fn success_with_pagination(data: T, pagination: PaginationMeta) -> Self {
        Self {
            success: Some(true),
            data: Some(data),
            error: None,
            meta: Some(ApiMeta {
                request_id: Some(uuid::Uuid::new_v4().to_string()),
                timestamp: Some(Utc::now()),
                pagination: Some(pagination),
            }),
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: Some(false),
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: None,
            }),
            meta: Some(ApiMeta::default()),
        }
    }

    pub fn error_with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            success: Some(false),
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            }),
            meta: Some(ApiMeta::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}

impl Default for ApiMeta {
    fn default() -> Self {
        Self {
            request_id: Some(uuid::Uuid::new_v4().to_string()),
            timestamp: Some(Utc::now()),
            pagination: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
    pub has_more: bool,
}

impl PaginationMeta {
    pub fn new(page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        Self {
            page,
            per_page,
            total,
            total_pages,
            has_more: page < total_pages,
        }
    }
}

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
