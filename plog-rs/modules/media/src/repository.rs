//! Media Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entity::*;

pub struct MediaRepository {
    db: Arc<DatabaseConnection>,
}

impl MediaRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Arc<DatabaseConnection> {
        &self.db
    }

    pub async fn find_all(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    pub async fn find_by_type(&self, mimetype: &str, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Mimetype.starts_with(mimetype))
            .order_by_desc(Column::CreatedAt)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_images(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        self.find_by_type("image/", page, per_page).await
    }

    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }

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
