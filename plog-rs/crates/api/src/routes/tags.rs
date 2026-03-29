//! 标签路由

use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Router, Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_core::types::ApiResponse;
use plog_content::{repository::TagRepository, entities::tag};

/// 创建标签路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v2/tags", get(list_tags).post(create_tag))
        .route("/api/v2/tags/:id", get(get_tag).put(update_tag).delete(delete_tag))
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<u64>,
    pub popular: Option<bool>,
}

/// 创建标签请求
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub tagname: String,
}

/// 更新标签请求
#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    pub tagname: Option<String>,
}

/// 标签列表
async fn list_tags(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = TagRepository::new(Arc::new(state.db));

    let result = if params.popular.unwrap_or(false) {
        let limit = params.limit.unwrap_or(20);
        repo.find_popular(limit).await
    } else {
        repo.find_all().await
    };

    match result {
        Ok(tags) => Json(ApiResponse::success(serde_json::to_value(tags).unwrap_or_default())),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 标签详情
async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = TagRepository::new(Arc::new(state.db));

    match repo.find_by_id(id).await {
        Ok(Some(tag)) => Json(ApiResponse::success(serde_json::to_value(tag).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Tag not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 创建标签
async fn create_tag(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(payload): Json<CreateTagRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = TagRepository::new(Arc::new(state.db));

    // 验证必填字段
    if payload.tagname.trim().is_empty() {
        return Json(ApiResponse::error("VALIDATION_ERROR", "Tag name is required"));
    }

    let new_tag = tag::ActiveModel {
        tagname: sea_orm::Set(payload.tagname),
        usenum: sea_orm::Set(0),
        ..Default::default()
    };

    match repo.create(new_tag).await {
        Ok(tag) => Json(ApiResponse::success(serde_json::to_value(tag).unwrap_or_default())),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 更新标签
async fn update_tag(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
    Json(payload): Json<UpdateTagRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = TagRepository::new(Arc::new(state.db));

    let mut update_data = tag::ActiveModel::default();
    
    if let Some(tagname) = payload.tagname {
        update_data.tagname = sea_orm::Set(tagname);
    }

    match repo.update(id, update_data).await {
        Ok(Some(tag)) => Json(ApiResponse::success(serde_json::to_value(tag).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Tag not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 删除标签
async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = TagRepository::new(Arc::new(state.db));

    match repo.delete(id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::error("NOT_FOUND", "Tag not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}
