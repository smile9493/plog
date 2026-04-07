//! 文章 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::post::*;

/// 文章 Repository
pub struct PostRepository {
    db: Arc<DatabaseConnection>,
}

impl PostRepository {
    /// 创建新的文章 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 根据 ID 获取文章
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    /// 根据别名获取文章
    pub async fn find_by_alias(&self, alias: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Alias.eq(alias))
            .one(&*self.db)
            .await
    }

    /// 获取已发布的文章
    pub async fn find_published(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Hide.eq("n"))
            .filter(Column::Type.eq("blog"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 获取分类下的文章
    pub async fn find_by_category(
        &self,
        category_id: i32,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .filter(Column::Sortid.eq(category_id))
            .filter(Column::Hide.eq("n"))
            .order_by_desc(Column::Date)
            .paginate(&*self.db, per_page);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 搜索文章
    pub async fn search(
        &self,
        keyword: &str,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
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
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 创建文章
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    /// 更新文章
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let post: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = post {
            let mut active: ActiveModel = model.into();
            
            // 更新字段 - 使用 is_set 检查并直接赋值
            if data.title.is_set() {
                active.title = data.title;
            }
            if data.content.is_set() {
                active.content = data.content;
            }
            if data.excerpt.is_set() {
                active.excerpt = data.excerpt;
            }
            if data.sortid.is_set() {
                active.sortid = data.sortid;
            }
            if data.cover.is_set() {
                active.cover = data.cover;
            }
            if data.hide.is_set() {
                active.hide = data.hide;
            }
            if data.top.is_set() {
                active.top = data.top;
            }
            if data.allow_remark.is_set() {
                active.allow_remark = data.allow_remark;
            }
            if data.password.is_set() {
                active.password = data.password;
            }

            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    /// 删除文章
    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 增加浏览量（原子操作）
    pub async fn increment_views(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::update_many()
            .col_expr(Column::Views, sea_orm::prelude::Expr::col(Column::Views).add(1))
            .filter(Column::Gid.eq(id))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected > 0)
    }

    /// 获取文章总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }

    /// 筛选文章（支持分类、关键词、状态、排序）
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

        // 按分类筛选
        if let Some(cat_id) = category_id {
            query = query.filter(Column::Sortid.eq(cat_id));
        }

        // 按关键词搜索
        if let Some(kw) = keyword {
            if !kw.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(Column::Title.contains(kw))
                        .add(Column::Content.contains(kw))
                );
            }
        }

        // 按状态筛选
        if let Some(st) = status {
            query = query.filter(Column::Hide.eq(st));
        } else {
            // 默认只显示已发布的
            query = query.filter(Column::Hide.eq("n"));
        }

        // 按类型筛选（默认只显示博客）
        query = query.filter(Column::Type.eq("blog"));

        // 排序
        match order {
            Some("views") => query = query.order_by_desc(Column::Views),
            Some("title") => query = query.order_by_asc(Column::Title),
            Some("date_asc") => query = query.order_by_asc(Column::Date),
            _ => query = query.order_by_desc(Column::Date),
        }

        let paginator = query.paginate(&*self.db, per_page);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }
}
