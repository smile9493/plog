//! 设置服务

use std::sync::Arc;
use std::collections::HashMap;

use plog_contracts::ApiResponse;
use sea_orm::*;

use crate::entity;
use crate::repository::SettingsRepository;

/// 设置服务
pub struct SettingsService {
    repo: SettingsRepository,
}

impl SettingsService {
    /// 创建新的设置服务
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: SettingsRepository::new(db),
        }
    }
    
    /// 获取所有设置
    pub async fn get_all(&self) -> ApiResponse<Vec<entity::Model>> {
        match self.repo.find_all().await {
            Ok(settings) => ApiResponse::success(settings),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 按分组获取设置
    pub async fn get_by_group(&self, group: &str) -> ApiResponse<Vec<entity::Model>> {
        match self.repo.find_by_group(group).await {
            Ok(settings) => ApiResponse::success(settings),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取单个设置
    pub async fn get(&self, key: &str) -> ApiResponse<entity::Model> {
        match self.repo.find_by_key(key).await {
            Ok(Some(setting)) => ApiResponse::success(setting),
            Ok(None) => ApiResponse::error("NOT_FOUND", "Setting not found"),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取设置值
    pub async fn get_value(&self, key: &str) -> ApiResponse<serde_json::Value> {
        match self.repo.get_value(key).await {
            Ok(Some(value)) => {
                match serde_json::from_str(&value) {
                    Ok(parsed) => ApiResponse::success(parsed),
                    Err(e) => ApiResponse::error("PARSE_ERROR", e.to_string()),
                }
            }
            Ok(None) => ApiResponse::error("NOT_FOUND", "Setting not found"),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 设置值
    pub async fn set_value(&self, key: &str, value: serde_json::Value) -> ApiResponse<entity::Model> {
        match self.repo.set_value(key, value).await {
            Ok(setting) => ApiResponse::success(setting),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 批量设置
    pub async fn set_values(&self, settings: HashMap<String, serde_json::Value>) -> ApiResponse<()> {
        match self.repo.set_values(settings).await {
            Ok(()) => ApiResponse::success(()),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 删除设置
    pub async fn delete(&self, key: &str) -> ApiResponse<bool> {
        match self.repo.delete(key).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 初始化默认设置
    pub async fn init_defaults(&self) -> ApiResponse<()> {
        match self.repo.init_defaults().await {
            Ok(()) => ApiResponse::success(()),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取站点名称
    pub async fn get_site_name(&self) -> String {
        self.repo
            .get_typed::<String>(entity::keys::SITE_NAME)
            .await
            .unwrap_or(None)
            .unwrap_or_else(|| "Plog CMS".to_string())
    }
    
    /// 获取站点 URL
    pub async fn get_site_url(&self) -> String {
        self.repo
            .get_typed::<String>(entity::keys::SITE_URL)
            .await
            .unwrap_or(None)
            .unwrap_or_else(|| "http://localhost".to_string())
    }
    
    /// 检查评论是否启用
    pub async fn is_comment_enabled(&self) -> bool {
        self.repo
            .get_typed::<bool>(entity::keys::COMMENT_ENABLED)
            .await
            .unwrap_or(None)
            .unwrap_or(true)
    }
    
    /// 获取最大上传大小
    pub async fn get_max_upload_size(&self) -> i64 {
        self.repo
            .get_typed::<i64>(entity::keys::UPLOAD_MAX_SIZE)
            .await
            .unwrap_or(None)
            .unwrap_or(10485760) // 10MB
    }
}
