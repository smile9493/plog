//! 统一 API 响应辅助

use axum::{response::{IntoResponse, Response}, Json};
use chrono::Utc;
use plog_contracts::{ApiMeta, ApiResponse, PaginationMeta};
use serde::Serialize;

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, crate::error::ApiErrorKind>;

pub fn ok<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        meta: Some(ApiMeta {
            request_id: request_id(),
            timestamp: Utc::now(),
            pagination: None,
        }),
    })
}

pub fn ok_with_pagination<T: Serialize>(data: T, pagination: PaginationMeta) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        meta: Some(ApiMeta {
            request_id: request_id(),
            timestamp: Utc::now(),
            pagination: Some(pagination),
        }),
    })
}

pub fn error<T>(code: impl Into<String>, message: impl Into<String>) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: false,
        data: None,
        error: Some(plog_contracts::ApiError {
            code: code.into(),
            message: message.into(),
            details: None,
        }),
        meta: Some(ApiMeta {
            request_id: request_id(),
            timestamp: Utc::now(),
            pagination: None,
        }),
    })
}

pub fn error_with_details<T>(
    code: impl Into<String>,
    message: impl Into<String>,
    details: serde_json::Value,
) -> Json<ApiResponse<T>> {
    Json(ApiResponse {
        success: false,
        data: None,
        error: Some(plog_contracts::ApiError {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }),
        meta: Some(ApiMeta {
            request_id: request_id(),
            timestamp: Utc::now(),
            pagination: None,
        }),
    })
}

pub fn request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub success: bool,
    pub message: String,
}

impl MessageResponse {
    pub fn ok(message: impl Into<String>) -> Self {
        Self { success: true, message: message.into() }
    }
}

impl IntoResponse for MessageResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
