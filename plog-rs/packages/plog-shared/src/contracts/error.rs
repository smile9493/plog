//! API Error Types
//! 
//! 统一的 API 错误定义

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(code: impl Into<String>, message: impl Into<String>, details: serde_json::Value) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }
    }
}

impl From<plog_core::CoreError> for ApiError {
    fn from(e: plog_core::CoreError) -> Self {
        Self {
            code: e.error_code().to_string(),
            message: e.to_string(),
            details: None,
        }
    }
}
