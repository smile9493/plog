//! 密码哈希服务

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString,
    },
    Argon2,
};
use plog_core::PlogResult;

/// 密码哈希服务
pub struct PasswordHasher;

impl PasswordHasher {
    /// 哈希密码
    pub fn hash(password: &str) -> PlogResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| {
                plog_core::PlogError::InternalError(format!("Password hashing error: {}", e))
            })
    }

    /// 验证密码
    pub fn verify(password: &str, hash: &str) -> PlogResult<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| {
            plog_core::PlogError::InternalError(format!("Invalid password hash: {}", e))
        })?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
