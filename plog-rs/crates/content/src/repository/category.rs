//! 分类 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::category::*;

/// 分类 Repository
pub struct CategoryRepository {
    db: Arc<DatabaseConnection>,
}

impl CategoryRepository {
    /// 创建新的分类 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 根据 ID 获取分类
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    /// 根据别名获取分类
    pub async fn find_by_alias(&self, alias: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Alias.eq(alias))
            .one(&*self.db)
            .await
    }

    /// 获取所有分类
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .order_by_asc(Column::Sortorder)
            .all(&*self.db)
            .await
    }

    /// 获取根分类
    pub async fn find_roots(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Pid.eq(0))
            .order_by_asc(Column::Sortorder)
            .all(&*self.db)
            .await
    }

    /// 获取子分类
    pub async fn find_children(&self, parent_id: i32) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Pid.eq(parent_id))
            .order_by_asc(Column::Sortorder)
            .all(&*self.db)
            .await
    }

    /// 创建分类
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    /// 更新分类
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let category: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = category {
            let mut active: ActiveModel = model.into();
            
            // 更新字段 - 使用 is_set 检查并直接赋值
            if data.sortname.is_set() {
                active.sortname = data.sortname;
            }
            if data.pid.is_set() {
                active.pid = data.pid;
            }
            if data.sortorder.is_set() {
                active.sortorder = data.sortorder;
            }
            if data.description.is_set() {
                active.description = data.description;
            }
            if data.alias.is_set() {
                active.alias = data.alias;
            }

            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    /// 删除分类
    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 获取分类总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
