//! 设置 Repository

use sea_orm::*;
use std::sync::Arc;
use std::collections::HashMap;

use crate::entity::*;

/// 设置 Repository
pub struct SettingsRepository {
    db: Arc<DatabaseConnection>,
}

impl SettingsRepository {
    /// 创建新的设置 Repository
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
    
    /// 获取所有设置
    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .order_by_asc(Column::Group)
            .order_by_asc(Column::Key)
            .all(&*self.db)
            .await
    }
    
    /// 按分组获取设置
    pub async fn find_by_group(&self, group: &str) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::Group.eq(group))
            .order_by_asc(Column::Key)
            .all(&*self.db)
            .await
    }
    
    /// 按键获取设置
    pub async fn find_by_key(&self, key: &str) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::Key.eq(key))
            .one(&*self.db)
            .await
    }
    
    /// 获取设置值
    pub async fn get_value(&self, key: &str) -> Result<Option<String>, DbErr> {
        let model = self.find_by_key(key).await?;
        Ok(model.map(|m| m.value))
    }
    
    /// 获取设置值并解析为指定类型
    pub async fn get_typed<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, DbErr> {
        let value = self.get_value(key).await?;
        match value {
            Some(v) => {
                let parsed: T = serde_json::from_str(&v)
                    .map_err(|e| DbErr::Custom(format!("Failed to parse setting: {}", e)))?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }
    
    /// 设置值
    pub async fn set_value(&self, key: &str, value: serde_json::Value) -> Result<Model, DbErr> {
        let value_str = serde_json::to_string(&value)
            .map_err(|e| DbErr::Custom(format!("Failed to serialize value: {}", e)))?;
        
        let now = chrono::Utc::now().timestamp();
        
        let existing = self.find_by_key(key).await?;
        
        match existing {
            Some(model) => {
                let mut active: ActiveModel = model.into();
                active.value = Set(value_str);
                active.updated_at = Set(now);
                active.update(&*self.db).await
            }
            None => {
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
    }
    
    /// 批量设置
    pub async fn set_values(&self, settings: HashMap<String, serde_json::Value>) -> Result<(), DbErr> {
        for (key, value) in settings {
            self.set_value(&key, value).await?;
        }
        Ok(())
    }
    
    /// 删除设置
    pub async fn delete(&self, key: &str) -> Result<bool, DbErr> {
        let result = Entity::delete_many()
            .filter(Column::Key.eq(key))
            .filter(Column::IsSystem.eq(false))
            .exec(&*self.db)
            .await?;
        Ok(result.rows_affected > 0)
    }
    
    /// 初始化默认设置
    pub async fn init_defaults(&self) -> Result<(), DbErr> {
        let defaults = crate::entity::default_settings();
        
        for (key, group, value, description) in defaults {
            let existing = self.find_by_key(key).await?;
            
            if existing.is_none() {
                let now = chrono::Utc::now().timestamp();
                let value_str = serde_json::to_string(&value)
                    .map_err(|e| DbErr::Custom(format!("Failed to serialize: {}", e)))?;
                
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
    
    /// 获取设置总数
    pub async fn count(&self) -> Result<u64, DbErr> {
        Entity::find().count(&*self.db).await
    }
}
