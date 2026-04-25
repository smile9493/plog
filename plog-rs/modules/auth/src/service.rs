//! Auth Service

use std::sync::Arc;
use sea_orm::*;
use plog_core::{CoreError, CoreResult};
use plog_content::{UserRepository, entities::user};

use crate::{JwtService, PasswordHasher, Claims};
use plog_shared::contracts::{UserInfo, LoginResponse, LoginRequest};

pub struct AuthService {
    db: Arc<DatabaseConnection>,
    jwt: Arc<JwtService>,
}

impl AuthService {
    pub fn new(db: Arc<DatabaseConnection>, jwt: Arc<JwtService>) -> Self {
        Self { db, jwt }
    }

    pub async fn login(&self, request: LoginRequest) -> CoreResult<LoginResponse> {
        let repo = UserRepository::new(self.db.clone());
        
        let user = repo.find_by_username(&request.username)
            .await
            .map_err(|e| CoreError::Database(format!("Database error: {}", e)))?;
        
        let user = user.ok_or_else(|| CoreError::Auth("Invalid username or password".into()))?;

        let is_valid = PasswordHasher::verify(&request.password, &user.password)?;
        if !is_valid {
            return Err(CoreError::Auth("Invalid username or password".into()));
        }

        let token = self.jwt.generate_token(user.uid, &user.username, &user.role)?;

        Ok(LoginResponse {
            token,
            expires_in: self.jwt.expiration(),
            user: UserInfo {
                id: user.uid,
                username: user.username,
                nickname: user.nickname,
                email: user.email,
                role: user.role,
                avatar: user.photo,
            },
        })
    }

    pub async fn get_current_user(&self, user_id: i32) -> CoreResult<UserInfo> {
        let repo = UserRepository::new(self.db.clone());
        
        let user = repo.find_by_id(user_id)
            .await
            .map_err(|e| CoreError::Database(format!("Database error: {}", e)))?;
        
        let user = user.ok_or_else(|| CoreError::NotFound("User not found".into()))?;

        Ok(UserInfo {
            id: user.uid,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            role: user.role,
            avatar: user.photo,
        })
    }

    pub async fn validate_token(&self, token: &str) -> CoreResult<Claims> {
        self.jwt.validate_token(token)
    }

    pub async fn change_password(&self, user_id: i32, old_password: &str, new_password: &str) -> CoreResult<()> {
        let repo = UserRepository::new(self.db.clone());
        
        let user = repo.find_by_id(user_id)
            .await
            .map_err(|e| CoreError::Database(format!("Database error: {}", e)))?;
        
        let user = user.ok_or_else(|| CoreError::NotFound("User not found".into()))?;

        let is_valid = PasswordHasher::verify(old_password, &user.password)?;
        if !is_valid {
            return Err(CoreError::Auth("Invalid old password".into()));
        }

        let hashed_password = PasswordHasher::hash(new_password)?;
        let mut user_active: user::ActiveModel = user.into();
        user_active.password = Set(hashed_password);
        user_active.update(&*self.db)
            .await
            .map_err(|e| CoreError::Database(format!("Database error: {}", e)))?;

        Ok(())
    }
}
