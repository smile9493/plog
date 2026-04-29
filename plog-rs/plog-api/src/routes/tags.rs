//! Tags Routes

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router, Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_shared::{ApiResponse, CrudRepository};
use plog_content::{TagRepository, entities::tag};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/tags", get(list_tags).post(create_tag))
        .route("/api/tags/:id", get(get_tag).delete(delete_tag))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<u64>,
    pub popular: Option<bool>,
}

#[tracing::instrument(skip(state))]
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
        Ok(tags) => Json(ApiResponse::ok(serde_json::to_value(tags).unwrap_or_default())),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

#[tracing::instrument(skip(state))]
async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = TagRepository::new(Arc::new(state.db));
    match repo.find_by_id(id).await {
        Ok(Some(tag)) => Json(ApiResponse::ok(serde_json::to_value(tag).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::err("NOT_FOUND", "Tag not found")),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

#[tracing::instrument(skip(state))]
async fn create_tag(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = TagRepository::new(Arc::new(state.db));
    let name = payload.get("tagname").and_then(|v| v.as_str()).unwrap_or("");
    
    let new_tag = tag::ActiveModel {
        tagname: sea_orm::Set(name.to_string()),
        usenum: sea_orm::Set(0),
        ..Default::default()
    };

    match repo.create(new_tag).await {
        Ok(tag) => Json(ApiResponse::ok(serde_json::to_value(tag).unwrap_or_default())),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}

#[tracing::instrument(skip(state))]
async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = TagRepository::new(Arc::new(state.db));
    match repo.delete(id).await {
        Ok(true) => Json(ApiResponse::ok(())),
        Ok(false) => Json(ApiResponse::err("NOT_FOUND", "Tag not found")),
        Err(e) => Json(ApiResponse::err("DATABASE_ERROR", e.to_string())),
    }
}
