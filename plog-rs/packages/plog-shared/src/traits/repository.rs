//! Repository Trait
//! 
//! 通用 Repository 接口定义，用于解耦数据访问层

use async_trait::async_trait;
use sea_orm::prelude::DbErr;
use std::sync::Arc;

#[async_trait]
pub trait Repository: Send + Sync {
    type Entity;
    type Model;
    type ActiveModel;
    
    fn db(&self) -> &Arc<sea_orm::DatabaseConnection>;
    
    async fn find_by_id(&self, id: i32) -> Result<Option<Self::Model>, DbErr>;
    
    async fn find_all(&self) -> Result<Vec<Self::Model>, DbErr>;
    
    async fn paginate(&self, page: u64, per_page: u64) -> Result<(Vec<Self::Model>, u64), DbErr>;
    
    async fn create(&self, data: Self::ActiveModel) -> Result<Self::Model, DbErr>;
    
    async fn update(&self, id: i32, data: Self::ActiveModel) -> Result<Option<Self::Model>, DbErr>;
    
    async fn delete(&self, id: i32) -> Result<bool, DbErr>;
    
    async fn count(&self) -> Result<u64, DbErr>;
}
