//! Settings Service

use std::sync::Arc;
use plog_shared::contracts::ApiResponse;

use crate::repository::SettingsRepository;

pub struct SettingsService {
    repo: SettingsRepository,
}

impl SettingsService {
    pub fn new(db: Arc<sea_orm::DatabaseConnection>) -> Self {
        Self {
            repo: SettingsRepository::new(db),
        }
    }

    pub async fn get_all(&self) -> ApiResponse<Vec<crate::entity::Model>> {
        match self.repo.find_all().await {
            Ok(settings) => ApiResponse::success(settings),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }

    pub async fn get(&self, key: &str) -> ApiResponse<crate::entity::Model> {
        match self.repo.find_by_key(key).await {
            Ok(Some(setting)) => ApiResponse::success(setting),
            Ok(None) => ApiResponse::error("NOT_FOUND", "Setting not found"),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }

    pub async fn set(&self, key: &str, value: serde_json::Value) -> ApiResponse<crate::entity::Model> {
        match self.repo.set_value(key, value).await {
            Ok(setting) => ApiResponse::success(setting),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }

    pub async fn get_site_name(&self) -> String {
        self.repo.find_by_key(crate::entity::keys::SITE_NAME)
            .await
            .ok()
            .flatten()
            .and_then(|m| serde_json::from_str(&m.value).ok())
            .unwrap_or_else(|| "Plog CMS".to_string())
    }
}
