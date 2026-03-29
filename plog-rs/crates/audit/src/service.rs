//! 审计日志服务

use std::sync::Arc;

use plog_contracts::{ApiResponse, PaginatedData, PaginationMeta};
use sea_orm::*;

use crate::entity;
use crate::repository::AuditRepository;

/// 审计日志服务
pub struct AuditService {
    repo: AuditRepository,
}

impl AuditService {
    /// 创建新的审计日志服务
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: AuditRepository::new(db),
        }
    }
    
    /// 记录日志
    pub async fn log(&self, data: entity::ActiveModel) -> ApiResponse<entity::Model> {
        match self.repo.log(data).await {
            Ok(log) => ApiResponse::success(log),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 记录创建操作
    pub async fn log_create(
        &self,
        user_id: i32,
        username: &str,
        resource_type: &str,
        resource_id: impl ToString,
        description: &str,
    ) -> ApiResponse<entity::Model> {
        let data = entity::AuditLogBuilder::new(entity::actions::CREATE, resource_type)
            .user(user_id, username)
            .resource_id(resource_id)
            .description(description)
            .build();
        self.log(data).await
    }
    
    /// 记录更新操作
    pub async fn log_update(
        &self,
        user_id: i32,
        username: &str,
        resource_type: &str,
        resource_id: impl ToString,
        description: &str,
        old_value: serde_json::Value,
        new_value: serde_json::Value,
    ) -> ApiResponse<entity::Model> {
        let data = entity::AuditLogBuilder::new(entity::actions::UPDATE, resource_type)
            .user(user_id, username)
            .resource_id(resource_id)
            .description(description)
            .old_value(old_value)
            .new_value(new_value)
            .build();
        self.log(data).await
    }
    
    /// 记录删除操作
    pub async fn log_delete(
        &self,
        user_id: i32,
        username: &str,
        resource_type: &str,
        resource_id: impl ToString,
        description: &str,
    ) -> ApiResponse<entity::Model> {
        let data = entity::AuditLogBuilder::new(entity::actions::DELETE, resource_type)
            .user(user_id, username)
            .resource_id(resource_id)
            .description(description)
            .build();
        self.log(data).await
    }
    
    /// 记录登录操作
    pub async fn log_login(
        &self,
        user_id: i32,
        username: &str,
        ip: &str,
        success: bool,
    ) -> ApiResponse<entity::Model> {
        let mut builder = entity::AuditLogBuilder::new(entity::actions::LOGIN, entity::resources::USER)
            .user(user_id, username)
            .resource_id(user_id)
            .ip_address(ip)
            .description(if success { "登录成功" } else { "登录失败" });
        
        if !success {
            builder = builder.failed("认证失败");
        }
        
        self.log(builder.build()).await
    }
    
    /// 获取日志列表
    pub async fn list(
        &self,
        page: u64,
        per_page: u64,
    ) -> ApiResponse<PaginatedData<entity::Model>> {
        match self.repo.find_all(page, per_page).await {
            Ok((items, total)) => {
                let total_pages = (total + per_page - 1) / per_page;
                let pagination = PaginationMeta {
                    page,
                    per_page,
                    total,
                    total_pages,
                    has_more: page < total_pages,
                };
                ApiResponse::success_with_pagination(
                    PaginatedData { items, pagination: pagination.clone() },
                    pagination,
                )
            }
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取用户日志
    pub async fn list_by_user(
        &self,
        user_id: i32,
        page: u64,
        per_page: u64,
    ) -> ApiResponse<PaginatedData<entity::Model>> {
        match self.repo.find_by_user(user_id, page, per_page).await {
            Ok((items, total)) => {
                let total_pages = (total + per_page - 1) / per_page;
                let pagination = PaginationMeta {
                    page,
                    per_page,
                    total,
                    total_pages,
                    has_more: page < total_pages,
                };
                ApiResponse::success_with_pagination(
                    PaginatedData { items, pagination: pagination.clone() },
                    pagination,
                )
            }
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取失败日志
    pub async fn list_failures(
        &self,
        page: u64,
        per_page: u64,
    ) -> ApiResponse<PaginatedData<entity::Model>> {
        match self.repo.find_failures(page, per_page).await {
            Ok((items, total)) => {
                let total_pages = (total + per_page - 1) / per_page;
                let pagination = PaginationMeta {
                    page,
                    per_page,
                    total,
                    total_pages,
                    has_more: page < total_pages,
                };
                ApiResponse::success_with_pagination(
                    PaginatedData { items, pagination: pagination.clone() },
                    pagination,
                )
            }
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 清理旧日志
    pub async fn cleanup(&self, days: i64) -> ApiResponse<u64> {
        let before = chrono::Utc::now().timestamp() - (days * 86400);
        match self.repo.delete_old(before).await {
            Ok(count) => ApiResponse::success(count),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
}
