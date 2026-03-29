//! JWT 服务

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use plog_core::PlogResult;
use serde::{Deserialize, Serialize};

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub sub: i32,
    /// 用户名
    pub username: String,
    /// 角色
    pub role: String,
    /// 过期时间
    pub exp: i64,
    /// 签发时间
    pub iat: i64,
}

/// JWT 服务
pub struct JwtService {
    secret: String,
    expiration: i64,
}

impl JwtService {
    /// 创建新的 JWT 服务
    pub fn new(secret: impl Into<String>, expiration: i64) -> Self {
        Self {
            secret: secret.into(),
            expiration,
        }
    }

    /// 生成 Token
    pub fn generate_token(&self, user_id: i32, username: &str, role: &str) -> PlogResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            username: username.to_string(),
            role: role.to_string(),
            exp: (now + Duration::seconds(self.expiration)).timestamp(),
            iat: now.timestamp(),
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());

        encode(&header, &claims, &encoding_key)
            .map_err(|e| plog_core::PlogError::InternalError(format!("JWT encoding error: {}", e)))
    }

    /// 验证 Token
    pub fn validate_token(&self, token: &str) -> PlogResult<Claims> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| plog_core::PlogError::AuthError(format!("Invalid token: {}", e)))
    }

    /// 刷新 Token
    pub fn refresh_token(&self, token: &str) -> PlogResult<String> {
        let claims = self.validate_token(token)?;
        self.generate_token(claims.sub, &claims.username, &claims.role)
    }
}
