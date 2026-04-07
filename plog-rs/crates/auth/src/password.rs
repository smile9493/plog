//! 密码哈希服务

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString,
    },
    Argon2, Params, Version,
};
use plog_core::PlogResult;

/// 密码哈希服务
pub struct PasswordHasher;

impl PasswordHasher {
    /// 获取推荐的 Argon2 参数
    fn params() -> Params {
        Params::new(Params::MIN_M_COST * 4, 3, 4, None).expect("Invalid Argon2 parameters")
    }

    /// 哈希密码
    pub fn hash(password: &str) -> PlogResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let params = Self::params();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

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

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, Self::params());

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "my_secure_password";
        let hash = PasswordHasher::hash(password).unwrap();

        assert!(hash.starts_with("$argon2id$"));

        let result = PasswordHasher::verify(password, &hash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_wrong_password() {
        let password = "correct_password";
        let hash = PasswordHasher::hash(password).unwrap();

        let result = PasswordHasher::verify("wrong_password", &hash).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let password = "same_password";
        let hash1 = PasswordHasher::hash(password).unwrap();
        let hash2 = PasswordHasher::hash(password).unwrap();

        assert_ne!(hash1, hash2);

        assert!(PasswordHasher::verify(password, &hash1).unwrap());
        assert!(PasswordHasher::verify(password, &hash2).unwrap());
    }

    #[test]
    fn test_invalid_hash_format() {
        let result = PasswordHasher::verify("password", "not-a-valid-hash");
        assert!(result.is_err());
    }
}
