//! Categories Routes

use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Router, Json,
};
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_shared::ApiResponse;
use plog_content::{CategoryRepository, entities::category};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/categories", get(list_categories).post(create_category))
        .route("/api/categories/:id", get(get_category).delete(delete_category))
}

async fn list_categories(
    State(state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));
    match repo.find_all().await {
        Ok(categories) => Json(ApiResponse::success(serde_json::to_value(categories).unwrap_or_default())),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));
    match repo.find_by_id(id).await {
        Ok(Some(category)) => Json(ApiResponse::success(serde_json::to_value(category).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Category not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

async fn create_category(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = CategoryRepository::new(Arc::new(state.db));
    let name = payload.get("sortname").and_then(|v| v.as_str()).unwrap_or("");
    
    let new_category = category::ActiveModel {
        sortname: sea_orm::Set(name.to_string()),
        pid: sea_orm::Set(0),
        sortorder: sea_orm::Set(0),
        ..Default::default()
    };

    match repo.create(new_category).await {
        Ok(category) => Json(ApiResponse::success(serde_json::to_value(category).unwrap_or_default())),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = CategoryRepository::new(Arc::new(state.db));
    match repo.delete(id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::error("NOT_FOUND", "Category not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}
