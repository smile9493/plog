//! Comment Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::comment::*;

pub struct CommentRepository {
    db: Arc<DatabaseConnection>,
}

impl CommentRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Arc<DatabaseConnection> {
        &self.db
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    pub async fn find_by_post(&self, post_id: i32, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Gid.eq(post_id))
            .filter(Column::Hide.eq("n"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_pending(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Hide.eq("y"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let comment = Entity::find_by_id(id).one(&*self.db).await?;
        if let Some(model) = comment {
            let mut active: ActiveModel = model.into();
            if data.content.is_set() { active.content = data.content.clone(); }
            if data.hide.is_set() { active.hide = data.hide.clone(); }
            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn approve(&self, id: i32) -> Result<bool, DbErr> {
        let comment = Entity::find_by_id(id).one(&*self.db).await?;
        if let Some(model) = comment {
            let mut active: ActiveModel = model.into();
            active.hide = Set("n".to_string());
            active.update(&*self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }

    pub async fn count_by_post(&self, post_id: i32) -> Result<u64, DbErr> {
        Entity::find()
            .filter(Column::Gid.eq(post_id))
            .filter(Column::Hide.eq("n"))
            .count(&*self.db)
            .await
    }
}
