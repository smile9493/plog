//! 认证路由

use axum::{routing::post, Router, Json, response::IntoResponse, http::{header, HeaderValue, StatusCode}};
use axum::response::Response;
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 修改密码请求
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

/// 创建认证路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", axum::routing::get(me))
        .route("/api/auth/refresh", post(refresh))
        .route("/api/auth/password", post(change_password))
        .route("/api/auth/profile", axum::routing::put(update_profile))
}

/// 创建 httpOnly cookie
fn create_token_cookie(token: &str, max_age: i64) -> HeaderValue {
    HeaderValue::from_str(&format!(
        "token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Strict",
        token, max_age
    )).unwrap_or_else(|_| HeaderValue::from_static(""))
}

/// 清除 token cookie
fn clear_token_cookie() -> HeaderValue {
    HeaderValue::from_static("token=; HttpOnly; Path=/; Max-Age=0; SameSite=Strict")
}

/// 登录处理
async fn login(
    state: axum::extract::State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.login(plog_auth::LoginRequest {
        username: payload.username,
        password: payload.password,
    }).await {
        Ok(response) => {
            let token = response.token.clone();
            let expires_in = response.expires_in;
            let body = Json(serde_json::json!({
                "success": true,
                "data": {
                    "user": response.user,
                    "expires_in": expires_in
                }
            }));
            let mut resp = (StatusCode::OK, body).into_response();
            resp.headers_mut().insert(header::SET_COOKIE, create_token_cookie(&token, expires_in));
            resp
        }
        Err(e) => {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "error": {
                        "code": e.error_code(),
                        "message": e.to_string()
                    }
                }))
            ).into_response()
        }
    }
}

/// 登出处理
async fn logout() -> Response {
    let body = Json(serde_json::json!({
        "success": true,
        "message": "Logged out successfully"
    }));
    let mut resp = (StatusCode::OK, body).into_response();
    resp.headers_mut().insert(header::SET_COOKIE, clear_token_cookie());
    resp
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

/// 刷新 Token
async fn refresh(
    state: axum::extract::State<AppState>,
    user: plog_auth::AuthUser,
) -> Json<serde_json::Value> {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.refresh_token_by_user_id(user.user_id).await {
        Ok(token) => {
            Json(serde_json::json!({
                "success": true,
                "data": { "token": token }
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

/// 修改密码
async fn change_password(
    state: axum::extract::State<AppState>,
    user: plog_auth::AuthUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Json<serde_json::Value> {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.change_password(user.user_id, &payload.old_password, &payload.new_password).await {
        Ok(_) => {
            Json(serde_json::json!({
                "success": true,
                "message": "Password changed successfully"
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

/// 更新用户资料
async fn update_profile(
    state: axum::extract::State<AppState>,
    user: plog_auth::AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.update_profile(user.user_id, payload).await {
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
