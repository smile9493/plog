//! 评论路由

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router, Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_core::types::ApiResponse;
use plog_content::{repository::CommentRepository, entities::comment};

/// 创建评论路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/comments", get(list_comments).post(create_comment))
        .route("/api/comments/:id", get(get_comment).put(update_comment).delete(delete_comment))
        .route("/api/comments/:id/approve", post(approve_comment))
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub post_id: Option<i32>,
    pub status: Option<String>,
}

/// 创建评论请求
#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub gid: i32,
    pub pid: Option<i32>,
    pub content: String,
    pub poster: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

/// 更新评论请求
#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: Option<String>,
    pub hide: Option<String>,
}

/// 评论列表
async fn list_comments(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CommentRepository::new(Arc::new(state.db));
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);

    let result = if let Some(post_id) = params.post_id {
        repo.find_by_post(post_id, page, per_page).await
    } else {
        repo.find_pending(page, per_page).await
    };

    match result {
        Ok((comments, total)) => {
            let total_pages = (total + per_page - 1) / per_page;
            let response_data = serde_json::json!({
                "items": comments,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total": total,
                    "total_pages": total_pages,
                    "has_more": page < total_pages
                }
            });
            Json(ApiResponse::success(response_data))
        }
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 评论详情
async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CommentRepository::new(Arc::new(state.db));

    match repo.find_by_id(id).await {
        Ok(Some(comment)) => Json(ApiResponse::success(serde_json::to_value(comment).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Comment not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 创建评论
async fn create_comment(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(payload): Json<CreateCommentRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CommentRepository::new(Arc::new(state.db));

    // 验证必填字段
    if payload.content.trim().is_empty() {
        return Json(ApiResponse::error("VALIDATION_ERROR", "Comment content is required"));
    }

    if payload.poster.trim().is_empty() {
        return Json(ApiResponse::error("VALIDATION_ERROR", "Poster name is required"));
    }

    let now = chrono::Utc::now().timestamp();
    let new_comment = comment::ActiveModel {
        gid: sea_orm::Set(payload.gid),
        pid: sea_orm::Set(payload.pid.unwrap_or(0)),
        content: sea_orm::Set(payload.content),
        poster: sea_orm::Set(payload.poster),
        email: sea_orm::Set(payload.email.unwrap_or_default()),
        url: sea_orm::Set(payload.url.unwrap_or_default()),
        ip: sea_orm::Set("0.0.0.0".to_string()),
        date: sea_orm::Set(now),
        hide: sea_orm::Set("y".to_string()), // 默认待审核
        ..Default::default()
    };

    match repo.create(new_comment).await {
        Ok(comment) => Json(ApiResponse::success(serde_json::to_value(comment).unwrap_or_default())),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 更新评论
async fn update_comment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
    Json(payload): Json<UpdateCommentRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CommentRepository::new(Arc::new(state.db));

    let mut update_data = comment::ActiveModel::default();
    
    if let Some(content) = payload.content {
        update_data.content = sea_orm::Set(content);
    }
    if let Some(hide) = payload.hide {
        update_data.hide = sea_orm::Set(hide);
    }

    match repo.update(id, update_data).await {
        Ok(Some(comment)) => Json(ApiResponse::success(serde_json::to_value(comment).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Comment not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 删除评论
async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = CommentRepository::new(Arc::new(state.db));

    match repo.delete(id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::error("NOT_FOUND", "Comment not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 审核评论
async fn approve_comment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = CommentRepository::new(Arc::new(state.db));

    match repo.approve(id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::error("NOT_FOUND", "Comment not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}
