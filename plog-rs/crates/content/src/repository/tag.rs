//! 标签 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::tag::*;

/// 标签 Repository
pub struct TagRepository {
    db: Arc<DatabaseConnection>,
}

impl TagRepository {
    /// 创建新的标签 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 根据 ID 获取标签
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    /// 根据名称获取标签
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Tagname.eq(name))
            .one(&*self.db)
            .await
    }

    /// 获取所有标签
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .order_by_desc(Column::Usenum)
            .all(&*self.db)
            .await
    }

    /// 获取热门标签
    pub async fn find_popular(&self, limit: u64) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Usenum.gt(0))
            .order_by_desc(Column::Usenum)
            .limit(limit)
            .all(&*self.db)
            .await
    }

    /// 创建标签
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    /// 更新标签
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let tag: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = tag {
            let mut active: ActiveModel = model.into();
            
            // 更新字段 - 使用 is_set 检查并直接赋值
            if data.tagname.is_set() {
                active.tagname = data.tagname;
            }
            if data.usenum.is_set() {
                active.usenum = data.usenum;
            }

            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    /// 删除标签
    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 增加使用次数
    pub async fn increment_usage(&self, id: i32) -> Result<bool, DbErr> {
        let tag: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = tag {
            let mut active: ActiveModel = model.into();
            active.usenum = Set(active.usenum.unwrap() + 1);
            active.update(&*self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 获取标签总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
