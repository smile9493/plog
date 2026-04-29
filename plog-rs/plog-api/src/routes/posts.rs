//! Posts Routes

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router, Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_shared::{ApiResponse, api_result, api_paged, api_delete, CrudRepository};
use plog_content::{PostRepository, entities::post};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(list_posts).post(create_post))
        .route("/api/posts/:id", get(get_post).put(update_post).delete(delete_post))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub category_id: Option<i32>,
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub order: Option<String>,
}

#[tracing::instrument(skip(state))]
async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<ApiResponse<plog_shared::PaginatedData<post::Model>>> {
    let repo = PostRepository::new(Arc::new(state.db));
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);

    Json(api_paged!(
        repo.filter(
            params.category_id,
            params.keyword.as_deref(),
            params.status.as_deref(),
            params.order.as_deref(),
            page,
            per_page,
        ).await,
        page,
        per_page
    ))
}

#[tracing::instrument(skip(state))]
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = PostRepository::new(Arc::new(state.db));
    Json(api_result!(
        repo.find_by_id(id).await.map(|opt| opt.map(|p| serde_json::to_value(p).unwrap_or_default())),
        "Post not found"
    ))
}

#[tracing::instrument(skip(state))]
async fn create_post(
    State(state): State<AppState>,
    _user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = PostRepository::new(Arc::new(state.db));
    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or("");
    
    let now = chrono::Utc::now().timestamp();
    let new_post = post::ActiveModel {
        title: sea_orm::Set(title.to_string()),
        content: sea_orm::Set(content.to_string()),
        author: sea_orm::Set(1),
        sortid: sea_orm::Set(0),
        date: sea_orm::Set(now),
        hide: sea_orm::Set("n".to_string()),
        r#type: sea_orm::Set("blog".to_string()),
        views: sea_orm::Set(0),
        comnum: sea_orm::Set(0),
        like_count: sea_orm::Set(0),
        top: sea_orm::Set("n".to_string()),
        sortop: sea_orm::Set("n".to_string()),
        allow_remark: sea_orm::Set("y".to_string()),
        ..Default::default()
    };

    Json(api_result!(
        repo.create(new_post).await.map(|p| Some(serde_json::to_value(p).unwrap_or_default())),
        "Failed to create post"
    ))
}

#[tracing::instrument(skip(_state, _payload))]
async fn update_post(
    State(_state): State<AppState>,
    Path(_id): Path<i32>,
    _user: AuthUser,
    Json(_payload): Json<serde_json::Value>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::err("NOT_IMPLEMENTED", "Update not implemented"))
}

#[tracing::instrument(skip(state))]
async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = PostRepository::new(Arc::new(state.db));
    Json(api_delete!(repo.delete(id).await, "Post not found"))
}
