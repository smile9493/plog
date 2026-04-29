//! User Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::user::*;
use plog_shared::{CrudRepository, impl_crud_repository, impl_sortable_repository};

pub struct UserRepository {
    pub db: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
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

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let user = Entity::find_by_id(id).one(&*self.db).await?;
        match user {
            Some(model) => {
                let mut active: ActiveModel = model.into();
                if data.password.is_set() { active.password = data.password.clone(); }
                if data.nickname.is_set() { active.nickname = data.nickname.clone(); }
                if data.email.is_set() { active.email = data.email.clone(); }
                if data.photo.is_set() { active.photo = data.photo.clone(); }
                if data.description.is_set() { active.description = data.description.clone(); }
                Ok(Some(active.update(&*self.db).await?))
            }
            None => Ok(None),
        }
    }
}

impl_crud_repository!(UserRepository, Entity, ActiveModel, i32, Column::Uid);
impl_sortable_repository!(UserRepository, Column::Uid);
