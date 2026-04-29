//! 媒体服务

use std::sync::Arc;

use plog_contracts::{ApiResponse, PaginatedData, PaginationMeta};
use sea_orm::*;

use crate::entity;
use crate::repository::MediaRepository;
use crate::storage::StorageDriver;

/// 媒体服务
pub struct MediaService {
    repo: MediaRepository,
    storage: Arc<dyn StorageDriver + Send + Sync>,
}

impl MediaService {
    /// 创建新的媒体服务
    pub fn new(db: Arc<DatabaseConnection>, storage: Arc<dyn StorageDriver + Send + Sync>) -> Self {
        Self {
            repo: MediaRepository::new(db),
            storage,
        }
    }
    
    /// 获取媒体列表
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
    
    /// 获取图片列表
    pub async fn list_images(
        &self,
        page: u64,
        per_page: u64,
    ) -> ApiResponse<PaginatedData<entity::Model>> {
        match self.repo.find_images(page, per_page).await {
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
    
    /// 获取媒体详情
    pub async fn get(&self, id: i32) -> ApiResponse<entity::Model> {
        match self.repo.find_by_id(id).await {
            Ok(Some(media)) => ApiResponse::success(media),
            Ok(None) => ApiResponse::error("NOT_FOUND", "Media not found"),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 上传文件
    pub async fn upload(
        &self,
        filename: &str,
        data: &[u8],
        mimetype: &str,
        user_id: Option<i32>,
    ) -> ApiResponse<entity::Model> {
        // 检查文件类型
        let ext = entity::get_extension(filename);
        let allowed = entity::default_allowed_extensions();
        if !allowed.contains(&ext.as_str()) {
            return ApiResponse::error("FILE_TYPE_NOT_ALLOWED", format!("File type .{} not allowed", ext));
        }
        
        // 生成唯一文件名
        let unique_name = entity::generate_unique_filename(filename);
        let filepath = format!("uploads/{}", unique_name);
        
        // 保存文件
        match self.storage.save(&filepath, data).await {
            Ok(_) => {
                let url = self.storage.url(&filepath);
                let now = chrono::Utc::now().timestamp();
                
                let active = entity::ActiveModel {
                    filename: Set(unique_name),
                    original_name: Set(filename.to_string()),
                    filepath: Set(filepath),
                    url: Set(url),
                    filesize: Set(data.len() as i64),
                    mimetype: Set(mimetype.to_string()),
                    extension: Set(ext),
                    // DEVIATION: Image dimensions not extracted - requires image parsing library
                    // Could be added with `image` crate if needed for thumbnails/resizing
                    width: Set(None),
                    height: Set(None),
                    user_id: Set(user_id),
                    storage_driver: Set("local".to_string()),
                    created_at: Set(now),
                    updated_at: Set(now),
                    ..Default::default()
                };
                
                match self.repo.create(active).await {
                    Ok(media) => ApiResponse::success(media),
                    Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
                }
            }
            Err(e) => ApiResponse::error("FILE_UPLOAD_FAILED", e.to_string()),
        }
    }
    
    /// 删除媒体
    pub async fn delete(&self, id: i32) -> ApiResponse<bool> {
        // 获取媒体信息
        let media = match self.repo.find_by_id(id).await {
            Ok(Some(m)) => m,
            Ok(None) => return ApiResponse::error("NOT_FOUND", "Media not found"),
            Err(e) => return ApiResponse::error("DATABASE_ERROR", e.to_string()),
        };
        
        // 删除文件
        if let Err(e) = self.storage.delete(&media.filepath).await {
            tracing::warn!("Failed to delete file: {}", e);
        }
        
        // 删除记录
        match self.repo.delete(id).await {
            Ok(result) => ApiResponse::success(result),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取媒体总数
    pub async fn count(&self) -> ApiResponse<u64> {
        match self.repo.count().await {
            Ok(count) => ApiResponse::success(count),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
    
    /// 获取总大小
    pub async fn total_size(&self) -> ApiResponse<i64> {
        match self.repo.total_size().await {
            Ok(size) => ApiResponse::success(size),
            Err(e) => ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    }
}
