//! Category Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::category::*;

pub struct CategoryRepository {
    db: Arc<DatabaseConnection>,
}

impl CategoryRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Arc<DatabaseConnection> {
        &self.db
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    pub async fn find_by_alias(&self, alias: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Alias.eq(alias))
            .one(&*self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .order_by_asc(Column::Sortorder)
            .all(&*self.db)
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

    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let category = Entity::find_by_id(id).one(&*self.db).await?;
        if let Some(model) = category {
            let mut active: ActiveModel = model.into();
            if data.sortname.is_set() { active.sortname = data.sortname.clone(); }
            if data.pid.is_set() { active.pid = data.pid.clone(); }
            if data.sortorder.is_set() { active.sortorder = data.sortorder.clone(); }
            if data.description.is_set() { active.description = data.description.clone(); }
            if data.alias.is_set() { active.alias = data.alias.clone(); }
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
