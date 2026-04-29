//! CRUD Repository Traits
//! 
//! 通用的 CRUD Repository 接口定义

use async_trait::async_trait;
use sea_orm::*;
use std::sync::Arc;

/// Sealed trait 模块 - 防止外部实现
pub mod private {
    pub trait Sealed {}
}

/// 通用 CRUD Repository Trait
///
/// 使用 sealed trait 模式防止外部实现，确保 API 稳定性
#[async_trait]
pub trait CrudRepository: Send + Sync + private::Sealed {
    type Entity: EntityTrait;
    type ActiveModel: ActiveModelBehavior + Send + Sync;
    type Id: Into<i32> + Copy + Send + Sync + 'static;

    fn db(&self) -> &Arc<DatabaseConnection>;
    fn id_column() -> <Self::Entity as EntityTrait>::Column;

    async fn find_by_id(&self, id: Self::Id) -> Result<Option<<Self::Entity as EntityTrait>::Model>, DbErr>;
    async fn find_all(&self) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, DbErr>;
    async fn create(&self, data: Self::ActiveModel) -> Result<<Self::Entity as EntityTrait>::Model, DbErr>;
    async fn delete(&self, id: Self::Id) -> Result<bool, DbErr>;
    async fn count(&self) -> Result<u64, DbErr>;
}

/// 支持排序的 Repository
#[async_trait]
pub trait SortableRepository: CrudRepository {
    fn default_sort_column() -> <Self::Entity as EntityTrait>::Column;
    async fn find_all_sorted(&self, desc: bool) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, DbErr>;
}
