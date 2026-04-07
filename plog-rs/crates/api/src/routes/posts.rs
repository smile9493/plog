//! 文章路由

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router, Json,
};
use serde::Deserialize;
use validator::Validate;
use std::sync::Arc;

use crate::AppState;
use plog_auth::AuthUser;
use plog_contracts::ApiResponse;
use plog_content::{repository::PostRepository, entities::post};

/// 创建文章路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/posts", get(list_posts).post(create_post))
        .route("/api/posts/:id", get(get_post).put(update_post).delete(delete_post))
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub category_id: Option<i32>,
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub order: Option<String>,
}

/// 创建文章请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1, max = 100000))]
    pub content: String,
    #[validate(length(max = 500))]
    pub excerpt: Option<String>,
    pub sortid: Option<i32>,
    pub cover: Option<String>,
    #[validate(length(max = 100))]
    pub alias: Option<String>,
    pub hide: Option<String>,
    pub top: Option<String>,
    pub allow_remark: Option<String>,
    #[validate(length(max = 100))]
    pub password: Option<String>,
    #[validate(length(max = 20))]
    pub r#type: Option<String>,
}

/// 更新文章请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 100000))]
    pub content: Option<String>,
    #[validate(length(max = 500))]
    pub excerpt: Option<String>,
    pub sortid: Option<i32>,
    pub cover: Option<String>,
    #[validate(length(max = 100))]
    pub alias: Option<String>,
    pub hide: Option<String>,
    pub top: Option<String>,
    pub allow_remark: Option<String>,
    #[validate(length(max = 100))]
    pub password: Option<String>,
}

/// 文章列表
async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = PostRepository::new(Arc::new(state.db));
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);

    let result = repo.filter(
        params.category_id,
        params.keyword.as_deref(),
        params.status.as_deref(),
        params.order.as_deref(),
        page,
        per_page,
    ).await;

    match result {
        Ok((posts, total)) => {
            let total_pages = (total + per_page - 1) / per_page;
            let response_data = serde_json::json!({
                "items": posts,
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

/// 文章详情
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = PostRepository::new(Arc::new(state.db));

    match repo.find_by_id(id).await {
        Ok(Some(post)) => Json(ApiResponse::success(serde_json::to_value(post).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Post not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 创建文章
async fn create_post(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreatePostRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    if let Err(e) = payload.validate() {
        let details: Vec<serde_json::Value> = e
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                let field = field.to_string();
                errors.iter().map(move |err| {
                    serde_json::json!({
                        "field": field,
                        "message": err.message.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "validation failed".to_string())
                    })
                })
            })
            .collect();
        return Json(ApiResponse::error_with_details(
            "VALIDATION_ERROR",
            "请求数据验证失败",
            serde_json::json!(details),
        ));
    }

    let repo = PostRepository::new(Arc::new(state.db));

    let now = chrono::Utc::now().timestamp();
    let new_post = post::ActiveModel {
        title: sea_orm::Set(payload.title),
        content: sea_orm::Set(payload.content),
        excerpt: sea_orm::Set(payload.excerpt),
        author: sea_orm::Set(user.user_id),
        sortid: sea_orm::Set(payload.sortid.unwrap_or(0)),
        date: sea_orm::Set(now),
        hide: sea_orm::Set(payload.hide.unwrap_or_else(|| "y".to_string())),
        r#type: sea_orm::Set(payload.r#type.unwrap_or_else(|| "blog".to_string())),
        views: sea_orm::Set(0),
        comnum: sea_orm::Set(0),
        like_count: sea_orm::Set(0),
        top: sea_orm::Set(payload.top.unwrap_or_else(|| "n".to_string())),
        sortop: sea_orm::Set("n".to_string()),
        allow_remark: sea_orm::Set(payload.allow_remark.unwrap_or_else(|| "y".to_string())),
        password: sea_orm::Set(payload.password),
        cover: sea_orm::Set(payload.cover),
        alias: sea_orm::Set(payload.alias),
        ..Default::default()
    };

    match repo.create(new_post).await {
        Ok(post) => Json(ApiResponse::success(serde_json::to_value(post).unwrap_or_default())),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 更新文章
async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
    Json(payload): Json<UpdatePostRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let repo = PostRepository::new(Arc::new(state.db));

    let mut update_data = post::ActiveModel::default();
    
    if let Some(title) = payload.title {
        update_data.title = sea_orm::Set(title);
    }
    if let Some(content) = payload.content {
        update_data.content = sea_orm::Set(content);
    }
    if let Some(excerpt) = payload.excerpt {
        update_data.excerpt = sea_orm::Set(Some(excerpt));
    }
    if let Some(sortid) = payload.sortid {
        update_data.sortid = sea_orm::Set(sortid);
    }
    if let Some(cover) = payload.cover {
        update_data.cover = sea_orm::Set(Some(cover));
    }
    if let Some(alias) = payload.alias {
        update_data.alias = sea_orm::Set(Some(alias));
    }
    if let Some(hide) = payload.hide {
        update_data.hide = sea_orm::Set(hide);
    }
    if let Some(top) = payload.top {
        update_data.top = sea_orm::Set(top);
    }
    if let Some(allow_remark) = payload.allow_remark {
        update_data.allow_remark = sea_orm::Set(allow_remark);
    }
    if let Some(password) = payload.password {
        update_data.password = sea_orm::Set(Some(password));
    }

    match repo.update(id, update_data).await {
        Ok(Some(post)) => Json(ApiResponse::success(serde_json::to_value(post).unwrap_or_default())),
        Ok(None) => Json(ApiResponse::error("NOT_FOUND", "Post not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}

/// 删除文章
async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    _user: AuthUser,
) -> Json<ApiResponse<()>> {
    let repo = PostRepository::new(Arc::new(state.db));

    match repo.delete(id).await {
        Ok(true) => Json(ApiResponse::success(())),
        Ok(false) => Json(ApiResponse::error("NOT_FOUND", "Post not found")),
        Err(e) => Json(ApiResponse::error("DATABASE_ERROR", e.to_string())),
    }
}
