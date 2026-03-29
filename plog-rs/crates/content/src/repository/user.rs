//! 用户 Repository

use sea_orm::*;
use std::sync::Arc;

use crate::entities::user::*;

/// 用户 Repository
pub struct UserRepository {
    db: Arc<DatabaseConnection>,
}

impl UserRepository {
    /// 创建新的用户 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// 根据 ID 获取用户
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    /// 根据用户名获取用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(&*self.db)
            .await
    }

    /// 根据邮箱获取用户
    pub async fn find_by_email(&self, email: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Email.eq(email))
            .one(&*self.db)
            .await
    }

    /// 获取所有用户
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(&*self.db).await
    }

    /// 分页获取用户
    pub async fn paginate(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<Model>, u64), DbErr> {
        let paginator = Entity::find()
            .order_by_asc(Column::Uid)
            .paginate(&*self.db, per_page);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 创建用户
    pub async fn create(&self, data: ActiveModel) -> Result<Model, DbErr> {
        data.insert(&*self.db).await
    }

    /// 更新用户
    pub async fn update(&self, id: i32, data: ActiveModel) -> Result<Option<Model>, DbErr> {
        let user: Option<Model> = Entity::find_by_id(id).one(&*self.db).await?;

        if let Some(model) = user {
            let mut active: ActiveModel = model.into();
            
            // 更新字段 - 使用 is_set 检查并直接赋值
            if data.password.is_set() {
                active.password = data.password;
            }
            if data.nickname.is_set() {
                active.nickname = data.nickname;
            }
            if data.email.is_set() {
                active.email = data.email;
            }
            if data.photo.is_set() {
                active.photo = data.photo;
            }
            if data.description.is_set() {
                active.description = data.description;
            }

            Ok(Some(active.update(&*self.db).await?))
        } else {
            Ok(None)
        }
    }

    /// 删除用户
    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        let result = Entity::delete_by_id(id).exec(&*self.db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 获取用户总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
