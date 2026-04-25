//! Password Hasher

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString},
    Argon2, Params, Version,
};
use plog_core::CoreResult;

pub struct PasswordHasher;

impl PasswordHasher {
    fn params() -> Params {
        Params::new(Params::MIN_M_COST * 4, 3, 4, None).expect("Invalid Argon2 parameters")
    }

    pub fn hash(password: &str) -> CoreResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let params = Self::params();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| plog_core::CoreError::Internal(format!("Password hashing error: {}", e)))
    }

    pub fn verify(password: &str, hash: &str) -> CoreResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| plog_core::CoreError::Internal(format!("Invalid password hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
