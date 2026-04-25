//! Tag Repository

use sea_orm::{*, prelude::Expr};
use std::sync::Arc;

use crate::entities::tag::*;

pub struct TagRepository {
    db: Arc<DatabaseConnection>,
}

impl TagRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Arc<DatabaseConnection> {
        &self.db
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Tagname.eq(name))
            .one(&*self.db)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .order_by_desc(Column::Usenum)
            .all(&*self.db)
            .await
    }

    pub async fn find_popular(&self, limit: u64) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Usenum.gt(0))
            .order_by_desc(Column::Usenum)
            .limit(limit)
            .all(&*self.db)
            .await
    }

    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let tag = Entity::find_by_id(id).one(&*self.db).await?;
        if let Some(model) = tag {
            let mut active: ActiveModel = model.into();
            if data.tagname.is_set() { active.tagname = data.tagname.clone(); }
            if data.usenum.is_set() { active.usenum = data.usenum.clone(); }
            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn increment_usage(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::update_many()
            .col_expr(Column::Usenum, Expr::col(Column::Usenum).add(1))
            .filter(Column::Tid.eq(id))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
