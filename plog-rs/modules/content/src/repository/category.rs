//! Category Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::category::*;
use plog_shared::{CrudRepository, impl_crud_repository, impl_sortable_repository};

pub struct CategoryRepository {
    pub db: Arc<DatabaseConnection>,
}

impl CategoryRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_by_alias(&self, alias: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Alias.eq(alias))
            .one(&*self.db)
            .await
    }

    pub async fn find_roots(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Pid.eq(0))
            .order_by_asc(Column::Sortorder)
            .all(&*self.db)
            .await
    }

    pub async fn find_children(&self, parent_id: i32) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Pid.eq(parent_id))
            .order_by_asc(Column::Sortorder)
            .all(&*self.db)
            .await
    }

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let category = Entity::find_by_id(id).one(&*self.db).await?;
        match category {
            Some(model) => {
                let mut active: ActiveModel = model.into();
                if data.sortname.is_set() { active.sortname = data.sortname.clone(); }
                if data.pid.is_set() { active.pid = data.pid.clone(); }
                if data.sortorder.is_set() { active.sortorder = data.sortorder.clone(); }
                if data.description.is_set() { active.description = data.description.clone(); }
                if data.alias.is_set() { active.alias = data.alias.clone(); }
                Ok(Some(active.update(&*self.db).await?))
            }
            None => Ok(None),
        }
    }
}

impl_crud_repository!(CategoryRepository, Entity, ActiveModel, i32, Column::Sid);
impl_sortable_repository!(CategoryRepository, Column::Sortorder);
