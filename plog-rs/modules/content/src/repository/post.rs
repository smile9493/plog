//! Post Repository

use sea_orm::{*, prelude::Expr};
use std::sync::Arc;

use crate::entities::post::*;
use plog_shared::apply_if_set;

pub struct PostRepository {
    db: Arc<DatabaseConnection>,
}

impl PostRepository {
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

    pub async fn paginate(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_published(&self, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Hide.eq("n"))
            .filter(Column::Type.eq("blog"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn find_by_category(&self, category_id: i32, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Sortid.eq(category_id))
            .filter(Column::Hide.eq("n"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }

    pub async fn search(&self, keyword: &str, page: u64, per_page: u64) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(
                Condition::any()
                    .add(Column::Title.contains(keyword))
                    .add(Column::Content.contains(keyword))
            )
            .filter(Column::Hide.eq("n"))
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
        let model = Entity::find_by_id(id).one(&*self.db).await?;
        match model {
            Some(m) => {
                let mut active: ActiveModel = m.into();
                apply_if_set!(data, active,
                    title, content, excerpt, sortid, cover, hide, top, allow_remark, password
                );
                Ok(Some(active.update(&*self.db).await?))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn increment_views(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::update_many()
            .col_expr(Column::Views, Expr::col(Column::Views).add(1))
            .filter(Column::Gid.eq(id))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }

    pub async fn filter(
        &self,
        category_id: Option<i32>,
        keyword: Option<&str>,
        status: Option<&str>,
        order: Option<&str>,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let mut query = Entity::find();

        if let Some(cat_id) = category_id {
            query = query.filter(Column::Sortid.eq(cat_id));
        }

        if let Some(kw) = keyword.filter(|k| !k.is_empty()) {
            query = query.filter(
                Condition::any()
                    .add(Column::Title.contains(kw))
                    .add(Column::Content.contains(kw))
            );
        }

        if let Some(st) = status.filter(|&s| s != "all") {
            query = query.filter(Column::Hide.eq(st));
        }

        query = match order {
            Some("views") => query.order_by_desc(Column::Views),
            Some("title") => query.order_by_asc(Column::Title),
            Some("date_asc") => query.order_by_asc(Column::Date),
            _ => query.order_by_desc(Column::Date),
        };

        let paginator = query.paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page.saturating_sub(1)).await?;
        Ok((items, total))
    }
}
