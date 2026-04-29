//! Auth Middleware

use axum::{
    extract::{FromRef, FromRequestParts, Request, State},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

use crate::{Claims, JwtService};
use plog_shared::Role;

#[derive(Clone)]
pub struct AuthState {
    pub jwt: Arc<JwtService>,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i32,
    pub username: String,
    pub role: Role,
}

impl AuthUser {
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    pub fn is_editor(&self) -> bool {
        self.role.is_editor()
    }

    pub fn has_role(&self, required: Role) -> bool {
        self.role.has_permission(required)
    }
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        let role = claims.role.parse().unwrap_or(Role::User);
        Self {
            user_id: claims.sub,
            username: claims.username,
            role,
        }
    }
}

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
        let claims = auth_state.jwt.validate_token(&token).map_err(|_| AuthError::InvalidToken)?;
        Ok(AuthUser::from(claims))
    }
}

fn extract_token(parts: &Parts) -> Result<String, AuthError> {
    extract_bearer_token(parts)
        .or_else(|| extract_cookie_token(parts))
        .ok_or(AuthError::MissingToken)
}

fn extract_bearer_token(parts: &Parts) -> Option<String> {
    parts.headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(str::to_string)
}

fn extract_cookie_token(parts: &Parts) -> Option<String> {
    parts.headers
        .get(axum::http::header::COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .map(str::trim)
                .find_map(|c| c.strip_prefix("token="))
        })
        .map(str::to_string)
}

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

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    InsufficientPermissions,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            Self::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
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
