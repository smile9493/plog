//! Settings Repository

use sea_orm::*;
use std::sync::Arc;
use std::collections::HashMap;

use crate::entity::*;

pub struct SettingsRepository {
    db: Arc<DatabaseConnection>,
}

impl SettingsRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &Arc<DatabaseConnection> {
        &self.db
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .order_by_asc(Column::Group)
            .order_by_asc(Column::Key)
            .all(&*self.db)
            .await
    }

    pub async fn find_by_group(&self, group: &str) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Group.eq(group))
            .order_by_asc(Column::Key)
            .all(&*self.db)
            .await
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Key.eq(key))
            .one(&*self.db)
            .await
    }

    pub async fn get_value(&self, key: &str) -> Result<Option<String>, DbErr> {
        let model = self.find_by_key(key).await?;
        Ok(model.map(|m| m.value))
    }

    pub async fn set_value(&self, key: &str, value: serde_json::Value) -> Result<Model, DbErr> {
        let value_str = serde_json::to_string(&value).map_err(|e| DbErr::Custom(format!("Failed to serialize: {}", e)))?;
        let now = chrono::Utc::now().timestamp();

        if let Some(model) = self.find_by_key(key).await? {
            let mut active: ActiveModel = model.into();
            active.value = Set(value_str);
            active.updated_at = Set(now);
            active.update(&*self.db).await
        } else {
            let active = ActiveModel {
                key: Set(key.to_string()),
                value: Set(value_str),
                group: Set("custom".to_string()),
                description: Set(None),
                is_system: Set(false),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            active.insert(&*self.db).await
        }
    }

    pub async fn delete(&self, key: &str) -> Result<bool, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::Key.eq(key))
            .filter(Column::IsSystem.eq(false))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn init_defaults(&self) -> Result<(), DbErr> {
        let defaults = crate::entity::default_settings();
        for (key, group, value, description) in defaults {
            if self.find_by_key(key).await?.is_none() {
                let now = chrono::Utc::now().timestamp();
                let value_str = serde_json::to_string(&value).map_err(|e| DbErr::Custom(format!("Failed to serialize: {}", e)))?;
                let active = ActiveModel {
                    key: Set(key.to_string()),
                    value: Set(value_str),
                    group: Set(group.to_string()),
                    description: Set(Some(description.to_string())),
                    is_system: Set(true),
                    created_at: Set(now),
                    updated_at: Set(now),
                    ..Default::default()
                };
                active.insert(&*self.db).await?;
            }
        }
        Ok(())
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
