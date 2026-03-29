//! 错误类型定义

use thiserror::Error;

/// Plog 错误类型
#[derive(Error, Debug)]
pub enum PlogError {
    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("数据库错误: {0}")]
    DatabaseError(String),

    #[error("认证错误: {0}")]
    AuthError(String),

    #[error("权限错误: {0}")]
    PermissionError(String),

    #[error("验证错误: {0}")]
    ValidationError(String),

    #[error("未找到: {0}")]
    NotFoundError(String),

    #[error("冲突: {0}")]
    ConflictError(String),

    #[error("内部错误: {0}")]
    InternalError(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

/// Plog 结果类型
pub type PlogResult<T> = Result<T, PlogError>;

impl PlogError {
    /// 获取 HTTP 状态码
    pub fn status_code(&self) -> u16 {
        match self {
            Self::ConfigError(_) => 500,
            Self::DatabaseError(_) => 500,
            Self::AuthError(_) => 401,
            Self::PermissionError(_) => 403,
            Self::ValidationError(_) => 422,
            Self::NotFoundError(_) => 404,
            Self::ConflictError(_) => 409,
            Self::InternalError(_) => 500,
            Self::Anyhow(_) => 500,
            Self::Io(_) => 500,
            Self::Json(_) => 400,
        }
    }

    /// 获取错误码
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::ConfigError(_) => "CONFIG_ERROR",
            Self::DatabaseError(_) => "DATABASE_ERROR",
            Self::AuthError(_) => "AUTH_ERROR",
            Self::PermissionError(_) => "PERMISSION_DENIED",
            Self::ValidationError(_) => "VALIDATION_ERROR",
            Self::NotFoundError(_) => "NOT_FOUND",
            Self::ConflictError(_) => "CONFLICT",
            Self::InternalError(_) => "INTERNAL_ERROR",
            Self::Anyhow(_) => "INTERNAL_ERROR",
            Self::Io(_) => "INTERNAL_ERROR",
            Self::Json(_) => "INVALID_JSON",
        }
    }
}
