//! Axum 认证中间件

use axum::{
    extract::{FromRef, FromRequestParts, Request, State},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

use crate::{Claims, JwtService};

/// 认证状态
#[derive(Clone)]
pub struct AuthState {
    pub jwt: Arc<JwtService>,
}

/// 已认证用户
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

impl AuthUser {
    /// 检查是否为管理员
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    /// 检查是否为编辑
    pub fn is_editor(&self) -> bool {
        self.role == "editor" || self.role == "admin"
    }

    /// 检查权限
    pub fn has_role(&self, required_role: &str) -> bool {
        match required_role {
            "admin" => self.role == "admin",
            "editor" => self.is_editor(),
            "user" => true,
            _ => false,
        }
    }
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            username: claims.username,
            role: claims.role,
        }
    }
}

/// 从请求中提取 AuthUser
#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_state = AuthState::from_ref(state);

        let token = extract_token(parts)?;
        let claims = auth_state
            .jwt
            .validate_token(&token)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthUser::from(claims))
    }
}

/// 从请求提取 Token（支持 Bearer header 和 Cookie）
fn extract_token(parts: &Parts) -> Result<String, AuthError> {
    // 优先从 Authorization header 提取
    let bearer_token = parts.headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(str::to_string);
    
    if let Some(token) = bearer_token {
        return Ok(token);
    }

    // 从 Cookie 提取
    let cookie_token = parts.headers
        .get(axum::http::header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .map(str::trim)
                .find_map(|cookie| cookie.strip_prefix("token="))
        })
        .map(str::to_string);

    cookie_token.ok_or(AuthError::MissingToken)
}

/// 认证中间件
pub async fn auth_middleware(
    State(state): State<AuthState>,
    request: Request,
    next: Next,
) -> Response {
    let (mut parts, body) = request.into_parts();

    let token = match extract_token(&parts) {
        Ok(token) => token,
        Err(err) => return err.into_response(),
    };

    match state.jwt.validate_token(&token) {
        Ok(claims) => {
            let user = AuthUser::from(claims);
            parts.extensions.insert(user);
            let request = Request::from_parts(parts, body);
            next.run(request).await
        }
        Err(_) => AuthError::InvalidToken.into_response(),
    }
}

/// 权限检查中间件工厂
pub fn require_role(required_role: &'static str) -> impl Fn(AuthUser, Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Clone {
    move |user: AuthUser, request: Request, next: Next| -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> {
        if !user.has_role(required_role) {
            Box::pin(async { AuthError::InsufficientPermissions.into_response() })
        } else {
            Box::pin(async move { next.run(request).await })
        }
    }
}

/// 认证错误类型
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    InsufficientPermissions,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AuthError::InsufficientPermissions => {
                (StatusCode::FORBIDDEN, "Insufficient permissions")
            }
        };

        let body = json!({
            "success": false,
            "error": {
                "code": status.as_u16(),
                "message": message
            }
        });

        (status, axum::Json(body)).into_response()
    }
}
