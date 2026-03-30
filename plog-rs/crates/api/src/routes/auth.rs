//! 认证路由

use axum::{routing::post, Router, Json};
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 创建认证路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", axum::routing::get(me))
}

/// 登录处理
async fn login(
    state: axum::extract::State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.login(plog_auth::LoginRequest {
        username: payload.username,
        password: payload.password,
    }).await {
        Ok(response) => {
            Json(serde_json::json!({
                "success": true,
                "data": response
            }))
        }
        Err(e) => {
            Json(serde_json::json!({
                "success": false,
                "error": {
                    "code": e.error_code(),
                    "message": e.to_string()
                }
            }))
        }
    }
}

/// 登出处理
async fn logout() -> Json<serde_json::Value> {
    // JWT 是无状态的，客户端只需删除 Token 即可登出
    Json(serde_json::json!({
        "success": true,
        "message": "Logged out successfully"
    }))
}

/// 获取当前用户信息
async fn me(
    state: axum::extract::State<AppState>,
    user: plog_auth::AuthUser,
) -> Json<serde_json::Value> {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.get_current_user(user.user_id).await {
        Ok(user_info) => {
            Json(serde_json::json!({
                "success": true,
                "data": user_info
            }))
        }
        Err(e) => {
            Json(serde_json::json!({
                "success": false,
                "error": {
                    "code": e.error_code(),
                    "message": e.to_string()
                }
            }))
        }
    }
}
