//! User Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::user::*;

pub struct UserRepository {
    db: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Arc<DatabaseConnection> {
        &self.db
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(&*self.db)
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Email.eq(email))
            .one(&*self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(&*self.db).await
    }

    pub async fn paginate(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_asc(Column::Uid)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let user = Entity::find_by_id(id).one(&*self.db).await?;
        if let Some(model) = user {
            let mut active: ActiveModel = model.into();
            if data.password.is_set() { active.password = data.password.clone(); }
            if data.nickname.is_set() { active.nickname = data.nickname.clone(); }
            if data.email.is_set() { active.email = data.email.clone(); }
            if data.photo.is_set() { active.photo = data.photo.clone(); }
            if data.description.is_set() { active.description = data.description.clone(); }
            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
