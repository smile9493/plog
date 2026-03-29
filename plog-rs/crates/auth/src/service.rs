//! 认证服务

use std::sync::Arc;

use plog_core::{PlogError, PlogResult};
use plog_content::entities::user;
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::{JwtService, PasswordHasher};

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

/// 用户信息
#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub role: String,
    pub email: Option<String>,
}

/// 认证服务
pub struct AuthService {
    db: Arc<DatabaseConnection>,
    jwt: Arc<JwtService>,
}

impl AuthService {
    /// 创建新的认证服务
    pub fn new(db: Arc<DatabaseConnection>, jwt: Arc<JwtService>) -> Self {
        Self { db, jwt }
    }

    /// 用户登录
    pub async fn login(&self, request: LoginRequest) -> PlogResult<LoginResponse> {
        // 查找用户
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(&request.username))
            .one(&*self.db)
            .await
            .map_err(|e| PlogError::DatabaseError(format!("Database error: {}", e)))?;

        let user = user.ok_or_else(|| PlogError::AuthError("Invalid username or password".into()))?;

        // 验证密码
        let is_valid = PasswordHasher::verify(&request.password, &user.password)?;
        if !is_valid {
            return Err(PlogError::AuthError("Invalid username or password".into()));
        }

        // 生成 JWT Token
        let token = self.jwt.generate_token(user.uid, &user.username, &user.role)?;

        Ok(LoginResponse {
            token,
            expires_in: 3600, // 1 hour
            user: UserInfo {
                id: user.uid,
                username: user.username,
                nickname: user.nickname,
                role: user.role,
                email: user.email,
            },
        })
    }

    /// 获取当前用户信息
    pub async fn get_current_user(&self, user_id: i32) -> PlogResult<UserInfo> {
        let user = user::Entity::find_by_id(user_id)
            .one(&*self.db)
            .await
            .map_err(|e| PlogError::DatabaseError(format!("Database error: {}", e)))?;

        let user = user.ok_or_else(|| PlogError::NotFoundError("User not found".into()))?;

        Ok(UserInfo {
            id: user.uid,
            username: user.username,
            nickname: user.nickname,
            role: user.role,
            email: user.email,
        })
    }

    /// 刷新 Token
    pub async fn refresh_token(&self, token: &str) -> PlogResult<String> {
        self.jwt.refresh_token(token)
    }

    /// 验证 Token
    pub async fn validate_token(&self, token: &str) -> PlogResult<crate::Claims> {
        self.jwt.validate_token(token)
    }
}
