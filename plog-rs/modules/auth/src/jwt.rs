//! JWT Service

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use plog_core::CoreResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService {
    secret: String,
    expiration: i64,
}

impl JwtService {
    pub fn new(secret: impl Into<String>, expiration: i64) -> Self {
        Self {
            secret: secret.into(),
            expiration,
        }
    }

    pub fn generate_token(&self, user_id: i32, username: &str, role: &str) -> CoreResult<String> {
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
            .map_err(|e| plog_core::CoreError::Auth(format!("JWT encoding error: {}", e)))
    }

    pub fn validate_token(&self, token: &str) -> CoreResult<Claims> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| plog_core::CoreError::Auth(format!("Invalid token: {}", e)))
    }

    pub fn refresh_token(&self, token: &str) -> CoreResult<String> {
        let claims = self.validate_token(token)?;
        self.generate_token(claims.sub, &claims.username, &claims.role)
    }

    pub fn expiration(&self) -> i64 {
        self.expiration
    }
}
