//! 评论 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::comment::*;

/// 评论 Repository
pub struct CommentRepository {
    db: Arc<DatabaseConnection>,
}

impl CommentRepository {
    /// 创建新的评论 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 根据 ID 获取评论
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    /// 获取文章的评论
    pub async fn find_by_post(
        &self,
        post_id: i32,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Gid.eq(post_id))
            .filter(Column::Hide.eq("n"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 获取待审核评论
    pub async fn find_pending(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Hide.eq("n"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 创建评论
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    /// 更新评论
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let comment: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = comment {
            let mut active: ActiveModel = model.into();
            
            // 更新字段 - 使用 is_set 检查并直接赋值
            if data.content.is_set() {
                active.content = data.content;
            }
            if data.hide.is_set() {
                active.hide = data.hide;
            }

            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    /// 审核评论
    pub async fn approve(&self, id: i32) -> Result<bool, DbErr> {
        let comment: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = comment {
            let mut active: ActiveModel = model.into();
            active.hide = Set("y".to_string());
            active.update(&*self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 删除评论
    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 获取评论总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }

    /// 获取文章评论数
    pub async fn count_by_post(&self, post_id: i32) -> Result<u64, DbErr> {
        Entity::find()
            .filter(Column::Gid.eq(post_id))
            .filter(Column::Hide.eq("n"))
            .count(&*self.db)
            .await
    }
}
