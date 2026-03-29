//! 审计日志 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entity::*;

/// 审计日志 Repository
pub struct AuditRepository {
    db: Arc<DatabaseConnection>,
}

impl AuditRepository {
    /// 创建新的审计日志 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
    
    /// 记录日志
    pub async fn log(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }
    
    /// 获取所有日志
    pub async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 按用户获取日志
    pub async fn find_by_user(
        &self,
        user_id: i32,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 按资源获取日志
    pub async fn find_by_resource(
        &self,
        resource_type: &str,
        resource_id: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::ResourceType.eq(resource_type))
            .filter(Column::ResourceId.eq(resource_id))
            .order_by_desc(Column::CreatedAt)
            .all(&*self.db)
            .await
    }
    
    /// 按操作类型获取日志
    pub async fn find_by_action(
        &self,
        action: &str,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Action.eq(action))
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 按状态获取日志
    pub async fn find_by_status(
        &self,
        status: &str,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Status.eq(status))
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 获取失败日志
    pub async fn find_failures(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        self.find_by_status(statuses::FAILED, page, per_page).await
    }
    
    /// 获取登录日志
    pub async fn find_logins(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        self.find_by_action(actions::LOGIN, page, per_page).await
    }
    
    /// 按时间范围获取日志
    pub async fn find_by_date_range(
        &self,
        start: i64,
        end: i64,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::CreatedAt.between(start, end))
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 删除旧日志
    pub async fn delete_old(&self, before: i64) -> Result<u64, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::CreatedAt.lt(before))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected)
    }
    
    /// 获取日志总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
