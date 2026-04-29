//! Comment Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::comment::*;
use plog_shared::{CrudRepository, impl_crud_repository, impl_sortable_repository};

pub struct CommentRepository {
    pub db: Arc<DatabaseConnection>,
}

impl CommentRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
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

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let comment = Entity::find_by_id(id).one(&*self.db).await?;
        match comment {
            Some(model) => {
                let mut active: ActiveModel = model.into();
                if data.content.is_set() { active.content = data.content.clone(); }
                if data.hide.is_set() { active.hide = data.hide.clone(); }
                Ok(Some(active.update(&*self.db).await?))
            }
            None => Ok(None),
        }
    }

    pub async fn approve(&self, id: i32) -> Result<bool, DbErr> {
        let comment = Entity::find_by_id(id).one(&*self.db).await?;
        match comment {
            Some(model) => {
                let mut active: ActiveModel = model.into();
                active.hide = Set("n".to_string());
                active.update(&*self.db).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    pub async fn count_by_post(&self, post_id: i32) -> Result<u64, DbErr> {
        Entity::find()
            .filter(Column::Gid.eq(post_id))
            .filter(Column::Hide.eq("n"))
            .count(&*self.db)
            .await
    }
}

impl_crud_repository!(CommentRepository, Entity, ActiveModel, i32, Column::Cid);
impl_sortable_repository!(CommentRepository, Column::Date);
