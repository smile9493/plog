//! Contracts 单元测试

use plog_contracts::*;

/// 测试成功响应
#[test]
fn test_api_response_success() {
    let response = ApiResponse::success("test data");

    assert!(response.success);
    assert_eq!(response.data, Some("test data"));
    assert!(response.error.is_none());
    assert!(response.meta.is_some());
}

/// 测试错误响应
#[test]
fn test_api_response_error() {
    let response: ApiResponse<()> = ApiResponse::error("TEST_ERROR", "Test error message");

    assert!(!response.success);
    assert!(response.data.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, "TEST_ERROR");
    assert_eq!(error.message, "Test error message");
}

/// 测试分页响应
#[test]
fn test_api_response_with_pagination() {
    let pagination = PaginationMeta {
        page: 1,
        per_page: 20,
        total: 100,
        total_pages: 5,
        has_more: true,
    };

    let response = ApiResponse::success_with_pagination(vec![1, 2, 3], pagination.clone());

    assert!(response.success);
    assert!(response.meta.is_some());

    let meta = response.meta.unwrap();
    assert!(meta.pagination.is_some());

    let pag = meta.pagination.unwrap();
    assert_eq!(pag.page, 1);
    assert_eq!(pag.total, 100);
}

/// 测试错误码
#[test]
fn test_error_codes() {
    assert_eq!(ErrorCode::BadRequest.as_str(), "BAD_REQUEST");
    assert_eq!(ErrorCode::Unauthorized.as_str(), "UNAUTHORIZED");
    assert_eq!(ErrorCode::NotFound.as_str(), "NOT_FOUND");
    assert_eq!(ErrorCode::InternalError.as_str(), "INTERNAL_ERROR");

    assert_eq!(ErrorCode::BadRequest.http_status(), 400);
    assert_eq!(ErrorCode::Unauthorized.http_status(), 401);
    assert_eq!(ErrorCode::NotFound.http_status(), 404);
    assert_eq!(ErrorCode::InternalError.http_status(), 500);
}

/// 测试分页参数
#[test]
fn test_pagination_params() {
    let mut params = PaginationParams::default();
    params.normalize();

    assert_eq!(params.page, 1);
    assert_eq!(params.per_page, 20);
    assert_eq!(params.offset(), 0);
    assert_eq!(params.total_pages(100), 5);
}

/// 测试分页参数限制
#[test]
fn test_pagination_params_limit() {
    let mut params = PaginationParams {
        page: 0,
        per_page: 200,
    };
    params.normalize();

    assert_eq!(params.page, 1);
    assert_eq!(params.per_page, 100);
}

/// 测试排序参数
#[test]
fn test_sort_params() {
    let sort = SortParams {
        field: "created_at".to_string(),
        order: SortOrder::Desc,
    };

    assert_eq!(sort.field, "created_at");
    assert_eq!(sort.order, SortOrder::Desc);
}

/// 测试权限定义
#[test]
fn test_permissions() {
    use plog_contracts::permission::permissions;

    assert_eq!(permissions::USER_VIEW, "user:view");
    assert_eq!(permissions::POST_CREATE, "post:create");
    assert_eq!(permissions::SETTINGS_UPDATE, "settings:update");
}

/// 测试角色权限映射
#[test]
fn test_role_permissions() {
    use plog_contracts::permission::{default_role_permissions, roles};

    let perms = default_role_permissions();

    // Admin 应该有所有权限
    let admin_perms = perms.get(roles::ADMIN).unwrap();
    assert!(admin_perms.contains(&"user:view"));
    assert!(admin_perms.contains(&"post:create"));
    assert!(admin_perms.contains(&"settings:update"));

    // Editor 应该有内容管理权限
    let editor_perms = perms.get(roles::EDITOR).unwrap();
    assert!(editor_perms.contains(&"post:create"));
    assert!(!editor_perms.contains(&"settings:update"));
}

/// 测试插件 manifest
#[test]
fn test_plugin_manifest() {
    let manifest = PluginManifest {
        id: "test-plugin".to_string(),
        name: "Test Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: Some("A test plugin".to_string()),
        author: Some("Test Author".to_string()),
        homepage: None,
        license: Some("MIT".to_string()),
        requires: Some("2.0.0".to_string()),
        dependencies: vec![],
        capabilities: vec![PluginCapability::AdminPage],
        permissions: vec!["post:view".to_string()],
        settings_schema: None,
        settings_defaults: None,
        menus: vec![],
        pages: vec![],
        jobs: vec![],
        events: vec![],
        migrations: vec![],
        admin_assets: None,
    };

    assert_eq!(manifest.id, "test-plugin");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.capabilities.len(), 1);
}

/// 测试主题 manifest
#[test]
fn test_theme_manifest() {
    let manifest = ThemeManifest {
        id: "test-theme".to_string(),
        name: "Test Theme".to_string(),
        version: "1.0.0".to_string(),
        description: Some("A test theme".to_string()),
        author: Some("Test Author".to_string()),
        homepage: None,
        license: Some("MIT".to_string()),
        requires: Some("2.0.0".to_string()),
        preview: Some("preview.jpg".to_string()),
        layouts: vec![LayoutDefinition {
            id: "default".to_string(),
            name: "Default Layout".to_string(),
            template: "layout.html".to_string(),
            default: true,
        }],
        slots: vec![],
        page_templates: vec![],
        settings_schema: None,
        settings_defaults: None,
        assets: None,
        supported_features: vec![ThemeFeature::Responsive, ThemeFeature::DarkMode],
    };

    assert_eq!(manifest.id, "test-theme");
    assert_eq!(manifest.layouts.len(), 1);
    assert_eq!(manifest.supported_features.len(), 2);
}
