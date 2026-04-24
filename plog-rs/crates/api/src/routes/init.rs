//! 初始化路由

use axum::{routing::post, Router, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct InitRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Debug, Serialize)]
pub struct InitStatusResponse {
    pub initialized: bool,
    pub message: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/init/status", post(check_init_status))
        .route("/api/init/setup", post(initialize_system))
}

async fn check_init_status(
    state: axum::extract::State<AppState>,
) -> Json<InitStatusResponse> {
    use plog_content::entities::user::Entity;
    use sea_orm::EntityTrait;
    
    let result = Entity::find()
        .one(&state.db)
        .await;
    
    match result {
        Ok(Some(_)) => Json(InitStatusResponse {
            initialized: true,
            message: "System already initialized".to_string(),
        }),
        Ok(None) => Json(InitStatusResponse {
            initialized: false,
            message: "System needs initialization".to_string(),
        }),
        Err(_) => Json(InitStatusResponse {
            initialized: false,
            message: "Unable to check initialization status".to_string(),
        }),
    }
}

async fn initialize_system(
    state: axum::extract::State<AppState>,
    Json(payload): Json<InitRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if payload.password.len() < 6 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    use plog_content::entities::user::{Entity, ActiveModel};
    use sea_orm::{EntityTrait, ActiveValue};
    
    let password_hash = plog_auth::PasswordHasher::hash(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let now = chrono::Utc::now().timestamp_millis();
    
    let active_user = ActiveModel {
        username: ActiveValue::Set(payload.username),
        password: ActiveValue::Set(password_hash),
        nickname: ActiveValue::Set(payload.nickname),
        role: ActiveValue::Set("admin".to_string()),
        email: ActiveValue::Set(None),
        photo: ActiveValue::Set(None),
        description: ActiveValue::Set(None),
        create_time: ActiveValue::Set(now),
        ..Default::default()
    };
    
    Entity::insert(active_user)
        .exec(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Admin user created successfully"
    })))
}
