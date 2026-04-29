//! Service Trait
//! 
//! 通用 Service 接口定义，用于解耦业务逻辑层

use async_trait::async_trait;
use plog_core::CoreError;
use crate::contracts::ApiResponse;

#[async_trait]
pub trait Service: Send + Sync {
    type Item;
    type List;
    
    async fn find(&self, id: i32) -> ApiResponse<Self::Item>;
    async fn list(&self, page: u64, per_page: u64) -> ApiResponse<Self::List>;
    async fn create(&self, data: impl serde::Serialize + Send + Sync) -> ApiResponse<Self::Item>;
    async fn update(&self, id: i32, data: impl serde::Serialize + Send + Sync) -> ApiResponse<Self::Item>;
    async fn delete(&self, id: i32) -> ApiResponse<()>;
}

pub trait ServiceError {
    fn to_api_response(&self) -> ApiResponse<()>;
}

impl ServiceError for CoreError {
    fn to_api_response(&self) -> ApiResponse<()> {
        ApiResponse::error(self.error_code(), self.to_string())
    }
}
