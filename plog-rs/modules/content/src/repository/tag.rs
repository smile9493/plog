//! Tag Repository

use sea_orm::{*, sea_query::Expr};
use std::sync::Arc;

use crate::entities::tag::*;
use plog_shared::{CrudRepository, impl_crud_repository, impl_sortable_repository};

pub struct TagRepository {
    pub db: Arc<DatabaseConnection>,
}

impl TagRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Tagname.eq(name))
            .one(&*self.db)
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

    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let tag = Entity::find_by_id(id).one(&*self.db).await?;
        match tag {
            Some(model) => {
                let mut active: ActiveModel = model.into();
                if data.tagname.is_set() { active.tagname = data.tagname.clone(); }
                if data.usenum.is_set() { active.usenum = data.usenum.clone(); }
                Ok(Some(active.update(&*self.db).await?))
            }
            None => Ok(None),
        }
    }

    pub async fn increment_usage(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::update_many()
            .col_expr(Column::Usenum, Expr::col(Column::Usenum).add(1))
            .filter(Column::Tid.eq(id))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected > 0)
    }
}

impl_crud_repository!(TagRepository, Entity, ActiveModel, i32, Column::Tid);
impl_sortable_repository!(TagRepository, Column::Usenum);
