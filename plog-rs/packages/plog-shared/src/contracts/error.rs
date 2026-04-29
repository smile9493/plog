//! API Error Types
//! 
//! 统一的 API 错误定义

pub use crate::contracts::response::ErrorBody;

/// 标准错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    ValidationError,
    InternalError,
    DatabaseError,
    AuthFailed,
    TokenInvalid,
    ResourceNotFound,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BadRequest => "BAD_REQUEST",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::InternalError => "INTERNAL_ERROR",
            Self::DatabaseError => "DATABASE_ERROR",
            Self::AuthFailed => "AUTH_FAILED",
            Self::TokenInvalid => "TOKEN_INVALID",
            Self::ResourceNotFound => "RESOURCE_NOT_FOUND",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::BadRequest | Self::ValidationError => 400,
            Self::Unauthorized | Self::AuthFailed | Self::TokenInvalid => 401,
            Self::Forbidden => 403,
            Self::NotFound | Self::ResourceNotFound => 404,
            Self::Conflict => 409,
            Self::InternalError | Self::DatabaseError => 500,
        }
    }
}
