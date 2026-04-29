//! 分类路由

use axum::{
    extract::{Path, State},
    routing::get,
    Router, Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_contracts::ApiResponse;
use plog_content::{repository::CategoryRepository, entities::category};
use plog_shared::CrudRepository;

/// 创建分类路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/categories", get(list_categories).post(create_category))
        .route("/api/categories/:id", get(get_category).put(update_category).delete(delete_category))
}

/// 创建分类请求
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub sortname: String,
    pub pid: Option<i32>,
    pub sortorder: Option<i32>,
    pub description: Option<String>,
    pub alias: Option<String>,
}

/// 更新分类请求
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub sortname: Option<String>,
    pub pid: Option<i32>,
    pub sortorder: Option<i32>,
    pub description: Option<String>,
    pub alias: Option<String>,
}

/// 分类列表
async fn list_categories(
    State(state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));

    match repo.find_all().await {
        Ok(categories) => Json(ApiResponse::ok(serde_json::to_value(categories).unwrap_or_default())),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

/// 分类详情
async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));

    match repo.find_by_id(id).await {
        Ok(Some(category)) => Json(ApiResponse::ok(serde_json::to_value(category).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::err("NOT_FOUND", "Category not found")),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

/// 创建分类
async fn create_category(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(payload): Json<CreateCategoryRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));

    // 验证必填字段
    if payload.sortname.trim().is_empty() {
        return Json(ApiResponse::err("VALIDATION_ERROR", "Category name is required"));
    }

    let new_category = category::ActiveModel {
        sortname: sea_orm::Set(payload.sortname),
        pid: sea_orm::Set(payload.pid.unwrap_or(0)),
        sortorder: sea_orm::Set(payload.sortorder.unwrap_or(0)),
        description: sea_orm::Set(payload.description),
        alias: sea_orm::Set(payload.alias),
        ..Default::default()
    };

    match repo.create(new_category).await {
        Ok(category) => Json(ApiResponse::ok(serde_json::to_value(category).unwrap_or_default())),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

/// 更新分类
async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));

    let mut update_data = category::ActiveModel::default();
    
    if let Some(sortname) = payload.sortname {
        update_data.sortname = sea_orm::Set(sortname);
    }
    if let Some(pid) = payload.pid {
        update_data.pid = sea_orm::Set(pid);
    }
    if let Some(sortorder) = payload.sortorder {
        update_data.sortorder = sea_orm::Set(sortorder);
    }
    if let Some(description) = payload.description {
        update_data.description = sea_orm::Set(Some(description));
    }
    if let Some(alias) = payload.alias {
        update_data.alias = sea_orm::Set(Some(alias));
    }

    match repo.update(id, update_data).await {
        Ok(Some(category)) => Json(ApiResponse::ok(serde_json::to_value(category).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::err("NOT_FOUND", "Category not found")),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

/// 删除分类
async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = CategoryRepository::new(Arc::new(state.db));

    match repo.delete(id).await {
        Ok(true) => Json(ApiResponse::ok(())),
        Ok(false) => Json(ApiResponse::err("NOT_FOUND", "Category not found")),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}
