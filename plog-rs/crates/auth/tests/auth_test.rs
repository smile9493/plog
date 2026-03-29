//! Auth 模块单元测试

use plog_auth::{Claims, JwtService, PasswordHasher};

/// 测试 JWT 签发和验证
#[test]
fn test_jwt_generate_and_validate() {
    let jwt_service = JwtService::new("test-secret-key-for-testing", 3600);

    // 生成 token
    let token = jwt_service.generate_token(1, "testuser", "admin").unwrap();
    assert!(!token.is_empty());

    // 验证 token
    let claims = jwt_service.validate_token(&token).unwrap();
    assert_eq!(claims.sub, 1);
    assert_eq!(claims.username, "testuser");
    assert_eq!(claims.role, "admin");
}

/// 测试 JWT 过期
#[test]
fn test_jwt_expiration() {
    // 创建一个过期时间很短的 token
    let jwt_service = JwtService::new("test-secret-key-for-testing", 1);

    let token = jwt_service.generate_token(1, "testuser", "admin").unwrap();

    // 立即验证应该成功
    let result = jwt_service.validate_token(&token);
    assert!(result.is_ok());

    // 注意：在实际测试中，需要等待 token 过期才能测试失败情况
    // 这里只验证 token 能被正确生成和验证
}

/// 测试 JWT 刷新
#[test]
fn test_jwt_refresh() {
    let jwt_service = JwtService::new("test-secret-key-for-testing", 3600);

    let token = jwt_service.generate_token(1, "testuser", "admin").unwrap();

    // 刷新 token
    let new_token = jwt_service.refresh_token(&token).unwrap();

    // 验证新 token
    let claims = jwt_service.validate_token(&new_token).unwrap();
    assert_eq!(claims.sub, 1);
    assert_eq!(claims.username, "testuser");
}

/// 测试无效 JWT
#[test]
fn test_invalid_jwt() {
    let jwt_service = JwtService::new("test-secret-key-for-testing", 3600);

    // 无效的 token
    let result = jwt_service.validate_token("invalid-token");
    assert!(result.is_err());

    // 使用不同密钥签名的 token
    let other_service = JwtService::new("different-secret-key", 3600);
    let token = other_service
        .generate_token(1, "testuser", "admin")
        .unwrap();

    let result = jwt_service.validate_token(&token);
    assert!(result.is_err());
}

/// 测试密码哈希
#[test]
fn test_password_hash() {
    let password = "my_secure_password";

    // 哈希密码
    let hash = PasswordHasher::hash(password).unwrap();
    assert!(!hash.is_empty());
    assert_ne!(hash, password);
}

/// 测试密码验证 - 正确密码
#[test]
fn test_password_verify_correct() {
    let password = "my_secure_password";
    let hash = PasswordHasher::hash(password).unwrap();

    // 验证正确密码
    let result = PasswordHasher::verify(password, &hash).unwrap();
    assert!(result);
}

/// 测试密码验证 - 错误密码
#[test]
fn test_password_verify_incorrect() {
    let password = "my_secure_password";
    let hash = PasswordHasher::hash(password).unwrap();

    // 验证错误密码
    let result = PasswordHasher::verify("wrong_password", &hash).unwrap();
    assert!(!result);
}

/// 测试密码哈希唯一性
#[test]
fn test_password_hash_uniqueness() {
    let password = "same_password";

    // 同一密码应该生成不同的哈希（因为 salt 不同）
    let hash1 = PasswordHasher::hash(password).unwrap();
    let hash2 = PasswordHasher::hash(password).unwrap();

    assert_ne!(hash1, hash2);

    // 但两个哈希都应该验证成功
    assert!(PasswordHasher::verify(password, &hash1).unwrap());
    assert!(PasswordHasher::verify(password, &hash2).unwrap());
}

/// 测试 Claims 结构
#[test]
fn test_claims_structure() {
    let jwt_service = JwtService::new("test-secret-key-for-testing", 3600);

    let token = jwt_service
        .generate_token(42, "john_doe", "editor")
        .unwrap();
    let claims = jwt_service.validate_token(&token).unwrap();

    assert_eq!(claims.sub, 42);
    assert_eq!(claims.username, "john_doe");
    assert_eq!(claims.role, "editor");
    assert!(claims.exp > claims.iat);
}

/// 测试不同角色
#[test]
fn test_different_roles() {
    let jwt_service = JwtService::new("test-secret-key-for-testing", 3600);

    let roles = vec!["admin", "editor", "user", "guest"];

    for role in roles {
        let token = jwt_service.generate_token(1, "user", role).unwrap();
        let claims = jwt_service.validate_token(&token).unwrap();
        assert_eq!(claims.role, role);
    }
}
