//! Unified App Error
//! 
//! 统一的应用错误枚举，支持自动转换 HTTP 响应

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::contracts::{AppResponse, ErrorBody};

/// 统一应用错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    
    #[error("{0}")]
    Unauthorized(String),
    
    #[error("{0}")]
    Forbidden(String),
    
    #[error("{0}")]
    NotFound(String),
    
    #[error("{0}")]
    Conflict(String),
    
    #[error("{message}")]
    Validation { message: String, details: serde_json::Value },
    
    #[error("{0}")]
    Internal(String),
    
    #[error("数据库错误: {0}")]
    Database(String),
    
    #[error("服务不可用")]
    ServiceUnavailable,
    
    #[error("{0}")]
    AuthFailed(String),
    
    #[error("Token 无效或已过期")]
    TokenInvalid,
    
    #[error("{0}")]
    ResourceNotFound(String),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("IO 错误: {0}")]
    Io(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) | Self::Validation { .. } => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) | Self::AuthFailed(_) | Self::TokenInvalid => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) | Self::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Internal(_) | Self::Database(_) | Self::Config(_) | Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::NotFound(_) | Self::ResourceNotFound(_) => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::Validation { .. } => "VALIDATION_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::Database(_) => "DATABASE_ERROR",
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            Self::AuthFailed(_) => "AUTH_FAILED",
            Self::TokenInvalid => "TOKEN_INVALID",
            Self::Config(_) => "CONFIG_ERROR",
            Self::Io(_) => "IO_ERROR",
        }
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::ResourceNotFound(resource.into())
    }

    pub fn validation(message: impl Into<String>, details: serde_json::Value) -> Self {
        Self::Validation {
            message: message.into(),
            details,
        }
    }

    pub fn into_body(self) -> ErrorBody {
        let code = self.error_code().to_string();
        match self {
            Self::Validation { message, details } => ErrorBody {
                code,
                message,
                details: Some(details),
            },
            _ => ErrorBody {
                code,
                message: self.to_string(),
                details: None,
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error = self.into_body();
        let body = AppResponse::<()> {
            success: false,
            data: None,
            error: Some(error),
            meta: None,
        };

        (status, Json(body)).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(e: sea_orm::DbErr) -> Self {
        Self::Database(e.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        let details = e.field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |err| {
                    serde_json::json!({
                        "field": field,
                        "message": err.message.as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "validation failed".to_string())
                    })
                })
            })
            .collect::<Vec<_>>();

        Self::validation("验证失败", serde_json::Value::Array(details))
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::BadRequest(e.to_string())
    }
}

impl From<config::ConfigError> for AppError {
    fn from(e: config::ConfigError) -> Self {
        Self::Config(e.to_string())
    }
}

impl From<plog_core::CoreError> for AppError {
    fn from(e: plog_core::CoreError) -> Self {
        match e {
            plog_core::CoreError::Auth(msg) => Self::AuthFailed(msg),
            plog_core::CoreError::Permission(msg) => Self::Forbidden(msg),
            plog_core::CoreError::NotFound(msg) => Self::ResourceNotFound(msg),
            plog_core::CoreError::Validation(msg) => Self::validation(msg, serde_json::json!({})),
            plog_core::CoreError::Conflict(msg) => Self::Conflict(msg),
            plog_core::CoreError::Database(msg) => Self::Database(msg),
            plog_core::CoreError::Config(msg) => Self::Config(msg),
            plog_core::CoreError::Io(msg) => Self::Io(msg),
            plog_core::CoreError::Internal(msg) => Self::Internal(msg),
        }
    }
}

/// 统一 Result 类型
pub type AppResult<T> = Result<T, AppError>;
