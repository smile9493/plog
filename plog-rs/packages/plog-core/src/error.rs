//! Plog CMS Core Error Types
//! 
//! 核心错误类型，所有模块共享的错误定义

use thiserror::Error;

/// Plog 核心错误类型
/// 
/// 这是应用程序的基础错误类型，所有其他模块的错误都应该能够转换为这个类型
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("配置错误: {0}")]
    Config(String),

    #[error("数据库错误: {0}")]
    Database(String),

    #[error("认证错误: {0}")]
    Auth(String),

    #[error("权限错误: {0}")]
    Permission(String),

    #[error("验证错误: {0}")]
    Validation(String),

    #[error("未找到: {0}")]
    NotFound(String),

    #[error("冲突: {0}")]
    Conflict(String),

    #[error("内部错误: {0}")]
    Internal(String),

    #[error("IO错误: {0}")]
    Io(String),
}

impl CoreError {
    /// 获取 HTTP 状态码
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Config(_) | Self::Database(_) | Self::Internal(_) => 500,
            Self::Auth(_) => 401,
            Self::Permission(_) => 403,
            Self::Validation(_) => 422,
            Self::NotFound(_) => 404,
            Self::Conflict(_) => 409,
            Self::Io(_) => 500,
        }
    }

    /// 获取错误码
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Config(_) => "CONFIG_ERROR",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Auth(_) => "AUTH_ERROR",
            Self::Permission(_) => "PERMISSION_DENIED",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::Io(_) => "IO_ERROR",
        }
    }
}

impl From<std::io::Error> for CoreError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(e: serde_json::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<config::ConfigError> for CoreError {
    fn from(e: config::ConfigError) -> Self {
        Self::Config(e.to_string())
    }
}

/// 核心 Result 类型
pub type CoreResult<T> = Result<T, CoreError>;
