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

    /// 获取过期时间
    pub fn expiration(&self) -> i64 {
        self.expiration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_jwt_service() -> JwtService {
        JwtService::new("test-secret-key-for-unit-testing", 3600)
    }

    #[test]
    fn test_generate_and_validate_token() {
        let jwt = create_jwt_service();

        let token = jwt.generate_token(1, "testuser", "admin").unwrap();
        assert!(!token.is_empty());

        let claims = jwt.validate_token(&token).unwrap();
        assert_eq!(claims.sub, 1);
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, "admin");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_validate_invalid_token() {
        let jwt = create_jwt_service();
        let result = jwt.validate_token("invalid-token-string");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_wrong_secret() {
        let jwt1 = JwtService::new("secret-one", 3600);
        let jwt2 = JwtService::new("secret-two", 3600);

        let token = jwt1.generate_token(1, "user", "admin").unwrap();
        let result = jwt2.validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token() {
        let jwt = create_jwt_service();

        let token = jwt.generate_token(1, "testuser", "admin").unwrap();
        let new_token = jwt.refresh_token(&token).unwrap();

        let claims = jwt.validate_token(&new_token).unwrap();
        assert_eq!(claims.sub, 1);
        assert_eq!(claims.username, "testuser");
    }

    #[test]
    fn test_expiration() {
        let jwt = JwtService::new("test", 7200);
        assert_eq!(jwt.expiration(), 7200);
    }
}
