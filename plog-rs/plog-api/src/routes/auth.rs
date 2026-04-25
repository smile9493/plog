//! Auth Routes

use axum::{
    routing::{post, get},
    Router, Json, response::IntoResponse,
    http::{header, HeaderValue, StatusCode},
};
use axum::extract::State;
use std::sync::Arc;

use crate::AppState;
use plog_shared::contracts::LoginRequest;
use plog_shared::ApiResponse;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(me))
}

fn create_token_cookie(token: &str, max_age: i64) -> HeaderValue {
    HeaderValue::from_str(&format!(
        "token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Strict",
        token, max_age
    )).unwrap_or_else(|_| HeaderValue::from_static(""))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.login(payload).await {
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
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "success": false,
                "error": {
                    "code": e.error_code(),
                    "message": e.to_string()
                }
            }))).into_response()
        }
    }
}

async fn logout() -> impl IntoResponse {
    let body = Json(serde_json::json!({
        "success": true,
        "message": "Logged out successfully"
    }));
    let mut resp = (StatusCode::OK, body).into_response();
    resp.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static("token=; HttpOnly; Path=/; Max-Age=0; SameSite=Strict")
    );
    resp
}

async fn me(
    State(state): State<AppState>,
    user: plog_auth::AuthUser,
) -> Json<ApiResponse<plog_shared::contracts::UserInfo>> {
    let auth_service = plog_auth::AuthService::new(
        Arc::new(state.db.clone()),
        state.jwt.clone(),
    );

    match auth_service.get_current_user(user.user_id).await {
        Ok(user_info) => Json(ApiResponse::success(user_info)),
        Err(e) => Json(ApiResponse::error(e.error_code(), e.to_string())),
    }
}
