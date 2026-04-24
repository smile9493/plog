//! API 错误类型

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use thiserror::Error;

use crate::response::{error_with_details, request_id};
use plog_contracts::ApiResponse;

#[derive(Debug, Clone, Serialize)]
pub struct PublicError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Error)]
pub enum ApiErrorKind {
    #[error("bad request")]
    BadRequest { code: &'static str, public_message: &'static str },
    #[error("unauthorized")]
    Unauthorized { code: &'static str, public_message: &'static str },
    #[error("forbidden")]
    Forbidden { code: &'static str, public_message: &'static str },
    #[error("not found")]
    NotFound { code: &'static str, public_message: &'static str },
    #[error("conflict")]
    Conflict { code: &'static str, public_message: &'static str },
    #[error("validation error")]
    Validation { code: &'static str, public_message: &'static str, details: serde_json::Value },
    #[error("internal error")]
    Internal { code: &'static str, public_message: &'static str },
}

impl ApiErrorKind {
    pub fn bad_request(message: impl Into<&'static str>) -> Self {
        Self::BadRequest { code: "BAD_REQUEST", public_message: message.into() }
    }
    pub fn unauthorized(message: impl Into<&'static str>) -> Self {
        Self::Unauthorized { code: "UNAUTHORIZED", public_message: message.into() }
    }
    pub fn forbidden(message: impl Into<&'static str>) -> Self {
        Self::Forbidden { code: "FORBIDDEN", public_message: message.into() }
    }
    pub fn not_found(message: impl Into<&'static str>) -> Self {
        Self::NotFound { code: "NOT_FOUND", public_message: message.into() }
    }
    pub fn conflict(message: impl Into<&'static str>) -> Self {
        Self::Conflict { code: "CONFLICT", public_message: message.into() }
    }
    pub fn validation(message: impl Into<&'static str>, details: serde_json::Value) -> Self {
        Self::Validation { code: "VALIDATION_ERROR", public_message: message.into(), details }
    }
    pub fn internal(message: impl Into<&'static str>) -> Self {
        Self::Internal { code: "INTERNAL_ERROR", public_message: message.into() }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            Self::Forbidden { .. } => StatusCode::FORBIDDEN,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::Conflict { .. } => StatusCode::CONFLICT,
            Self::Validation { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiErrorKind {
    fn into_response(self) -> Response {
        match self {
            Self::Validation { code, public_message, details } => {
                (self.status_code(), error_with_details::<serde_json::Value>(code, public_message, details)).into_response()
            }
            Self::BadRequest { code, public_message }
            | Self::Unauthorized { code, public_message }
            | Self::Forbidden { code, public_message }
            | Self::NotFound { code, public_message }
            | Self::Conflict { code, public_message }
            | Self::Internal { code, public_message } => {
                (self.status_code(), crate::response::error::<serde_json::Value>(code, public_message)).into_response()
            }
        }
    }
}

impl From<plog_core::PlogError> for ApiErrorKind {
    fn from(err: plog_core::PlogError) -> Self {
        use plog_core::PlogError::*;
        match err {
            ValidationError(_) => Self::validation("请求数据验证失败", serde_json::json!({})),
            NotFoundError(_) => Self::not_found("资源不存在"),
            ConflictError(_) => Self::conflict("资源冲突"),
            AuthError(_) => Self::unauthorized("认证失败"),
            PermissionError(_) => Self::forbidden("无权限访问"),
            ConfigError(_) | DatabaseError(_) | InternalError(_) => Self::internal("内部错误"),
            Anyhow(_) | Io(_) => Self::internal("内部错误"),
            Json(_) => Self::bad_request("请求参数错误"),
        }
    }
}

impl From<sea_orm::DbErr> for ApiErrorKind {
    fn from(_err: sea_orm::DbErr) -> Self {
        Self::internal("数据库错误")
    }
}

impl From<validator::ValidationErrors> for ApiErrorKind {
    fn from(err: validator::ValidationErrors) -> Self {
        let details = err.field_errors().iter().flat_map(|(field, errors)| {
            let field = field.to_string();
            errors.iter().map(move |error| {
                serde_json::json!({
                    "field": field,
                    "message": error.message.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "validation failed".to_string())
                })
            })
        }).collect::<Vec<_>>();

        Self::validation("请求数据验证失败", serde_json::Value::Array(details))
    }
}
