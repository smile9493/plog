//! Audit 单元测试

use plog_audit::*;

/// 测试操作类型常量
#[test]
fn test_action_constants() {
    use entity::actions;

    assert_eq!(actions::CREATE, "create");
    assert_eq!(actions::UPDATE, "update");
    assert_eq!(actions::DELETE, "delete");
    assert_eq!(actions::LOGIN, "login");
    assert_eq!(actions::LOGOUT, "logout");
}

/// 测试资源类型常量
#[test]
fn test_resource_constants() {
    use entity::resources;

    assert_eq!(resources::USER, "user");
    assert_eq!(resources::POST, "post");
    assert_eq!(resources::CATEGORY, "category");
    assert_eq!(resources::COMMENT, "comment");
    assert_eq!(resources::MEDIA, "media");
    assert_eq!(resources::SETTING, "setting");
}

/// 测试状态常量
#[test]
fn test_status_constants() {
    use entity::statuses;

    assert_eq!(statuses::SUCCESS, "success");
    assert_eq!(statuses::FAILED, "failed");
    assert_eq!(statuses::PENDING, "pending");
}

/// 测试审计日志构建器
#[test]
fn test_audit_log_builder() {
    use entity::AuditLogBuilder;

    let data = AuditLogBuilder::new(entity::actions::CREATE, entity::resources::POST)
        .user(1, "admin")
        .resource_id(42)
        .description("创建文章")
        .build();

    assert_eq!(data.action.as_ref(), "create");
    assert_eq!(data.resource_type.as_ref(), "post");
    assert_eq!(data.user_id.as_ref(), &Some(1));
    assert_eq!(data.username.as_ref(), &Some("admin".to_string()));
    assert_eq!(data.resource_id.as_ref(), &Some("42".to_string()));
    assert_eq!(data.description.as_ref(), "创建文章");
    assert_eq!(data.status.as_ref(), "success");
}

/// 测试审计日志构建器 - 失败状态
#[test]
fn test_audit_log_builder_failed() {
    use entity::AuditLogBuilder;

    let data = AuditLogBuilder::new(entity::actions::LOGIN, entity::resources::USER)
        .user(1, "admin")
        .ip_address("192.168.1.1")
        .failed("密码错误")
        .build();

    assert_eq!(data.status.as_ref(), "failed");
    assert_eq!(data.error_message.as_ref(), &Some("密码错误".to_string()));
    assert_eq!(data.ip_address.as_ref(), &Some("192.168.1.1".to_string()));
}

/// 测试审计日志构建器 - 带变更值
#[test]
fn test_audit_log_builder_with_values() {
    use entity::AuditLogBuilder;

    let old = serde_json::json!({"title": "旧标题"});
    let new = serde_json::json!({"title": "新标题"});

    let data = AuditLogBuilder::new(entity::actions::UPDATE, entity::resources::POST)
        .user(1, "admin")
        .resource_id(42)
        .old_value(old)
        .new_value(new)
        .description("更新文章标题")
        .build();

    assert!(data.old_value.as_ref().is_some());
    assert!(data.new_value.as_ref().is_some());
}

/// 测试审计日志构建器 - 请求信息
#[test]
fn test_audit_log_builder_with_request() {
    use entity::AuditLogBuilder;

    let data = AuditLogBuilder::new(entity::actions::CREATE, entity::resources::POST)
        .request("POST", "/api/v2/posts")
        .user_agent("Mozilla/5.0")
        .build();

    assert_eq!(data.request_method.as_ref(), &Some("POST".to_string()));
    assert_eq!(
        data.request_path.as_ref(),
        &Some("/api/v2/posts".to_string())
    );
    assert_eq!(data.user_agent.as_ref(), &Some("Mozilla/5.0".to_string()));
}
