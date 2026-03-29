//! 媒体 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entity::*;

/// 媒体 Repository
pub struct MediaRepository {
    db: Arc<DatabaseConnection>,
}

impl MediaRepository {
    /// 创建新的媒体 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
    
    /// 获取所有媒体
    pub async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 按用户获取媒体
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
    
    /// 按类型获取媒体
    pub async fn find_by_type(
        &self,
        mimetype: &str,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Mimetype.starts_with(mimetype))
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 获取图片
    pub async fn find_images(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        self.find_by_type("image/", page, per_page).await
    }
    
    /// 获取文档
    pub async fn find_documents(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(
                Condition::any()
                    .add(Column::Mimetype.eq("application/pdf"))
                    .add(Column::Mimetype.contains("document"))
                    .add(Column::Mimetype.contains("spreadsheet"))
            )
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;
        
        Ok((items, total))
    }
    
    /// 按 ID 获取
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }
    
    /// 创建媒体记录
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }
    
    /// 更新媒体记录
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let media = self.find_by_id(id).await?;
        
        if let Some(model) = media {
            let mut active: ActiveModel = model.into();
            
            if data.original_name.is_set() {
                active.original_name = data.original_name;
            }
            if data.filename.is_set() {
                active.filename = data.filename;
            }
            
            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }
    
    /// 删除媒体记录
    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }
    
    /// 获取总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
    
    /// 获取总大小
    pub async fn total_size(&self) -> Result<i64, DbErr> {
        let result = Entity::find()
            .select_only()
            .column_as(Column::Filesize.sum(), "total")
            .into_tuple::<Option<i64>>()
            .one(&*self.db)
            .await?;
        
        Ok(result.flatten().unwrap_or(0))
    }
}
