//! Plugin 单元测试

use plog_plugin::*;
use std::path::Path;

/// 测试插件管理器创建
#[test]
fn test_plugin_manager_new() {
    let manager = PluginManager::new("/tmp/plugins");
    assert_eq!(manager.count(), 0);
}

/// 测试插件状态
#[test]
fn test_plugin_status() {
    assert_eq!(PluginStatus::Active, PluginStatus::Active);
    assert_ne!(PluginStatus::Active, PluginStatus::Disabled);
    assert_ne!(PluginStatus::Installed, PluginStatus::Active);
}

/// 测试插件 manifest 解析
#[test]
fn test_manifest_parse() {
    let toml_str = r#"
id = "test-plugin"
name = "Test Plugin"
version = "1.0.0"
description = "A test plugin"
author = "Test Author"
license = "MIT"

capabilities = ["admin_page"]

[[menus]]
id = "test-menu"
title = "Test Menu"
route = "/admin/test"

[[pages]]
id = "test-page"
title = "Test Page"
route = "/admin/test"
component = "TestPage.vue"

[[events]]
event = "post_save"
handler = "on_post_save"
priority = 10
"#;

    let manifest: PluginManifest = toml::from_str(toml_str).unwrap();

    assert_eq!(manifest.id, "test-plugin");
    assert_eq!(manifest.name, "Test Plugin");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.menus.len(), 1);
    assert_eq!(manifest.pages.len(), 1);
    assert_eq!(manifest.events.len(), 1);
}

/// 测试插件信息创建
#[test]
fn test_plugin_info_from_manifest() {
    let manifest = PluginManifest {
        id: "test-plugin".to_string(),
        name: "Test Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test".to_string()),
        author: Some("Test".to_string()),
        homepage: None,
        license: Some("MIT".to_string()),
        requires: None,
        dependencies: vec![],
        capabilities: vec![PluginCapability::AdminPage],
        permissions: vec![],
        settings_schema: None,
        settings_defaults: None,
        menus: vec![],
        pages: vec![],
        jobs: vec![],
        events: vec![],
        migrations: vec![],
        admin_assets: None,
    };

    let info = PluginInfo::from_manifest(manifest, "/tmp/plugins/test-plugin");

    assert_eq!(info.manifest.id, "test-plugin");
    assert_eq!(info.status, PluginStatus::Installed);
    assert!(info.installed_at.is_some());
}

/// 测试插件能力检查
#[test]
fn test_plugin_has_capability() {
    let manifest = PluginManifest {
        id: "test-plugin".to_string(),
        name: "Test".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        author: None,
        homepage: None,
        license: None,
        requires: None,
        dependencies: vec![],
        capabilities: vec![PluginCapability::AdminPage, PluginCapability::ApiEndpoint],
        permissions: vec![],
        settings_schema: None,
        settings_defaults: None,
        menus: vec![],
        pages: vec![],
        jobs: vec![],
        events: vec![],
        migrations: vec![],
        admin_assets: None,
    };

    let info = PluginInfo::from_manifest(manifest, "");

    assert!(info.has_capability(&PluginCapability::AdminPage));
    assert!(info.has_capability(&PluginCapability::ApiEndpoint));
    assert!(!info.has_capability(&PluginCapability::ScheduledJob));
}

/// 测试菜单注册
#[test]
fn test_menu_registration() {
    let menu = MenuRegistration {
        id: "test-menu".to_string(),
        title: "Test Menu".to_string(),
        icon: Some("icon-test".to_string()),
        parent: None,
        route: "/admin/test".to_string(),
        order: 10,
        permission: Some("test:view".to_string()),
    };

    assert_eq!(menu.id, "test-menu");
    assert_eq!(menu.route, "/admin/test");
}

/// 测试页面注册
#[test]
fn test_page_registration() {
    let page = PageRegistration {
        id: "test-page".to_string(),
        title: "Test Page".to_string(),
        route: "/admin/test".to_string(),
        component: "TestPage.vue".to_string(),
        layout: Some("admin".to_string()),
        permission: Some("test:view".to_string()),
    };

    assert_eq!(page.id, "test-page");
    assert_eq!(page.component, "TestPage.vue");
}

/// 测试事件订阅
#[test]
fn test_event_subscription() {
    let event = EventSubscription {
        event: "post_save".to_string(),
        handler: "on_post_save".to_string(),
        priority: 10,
    };

    assert_eq!(event.event, "post_save");
    assert_eq!(event.priority, 10);
}

/// 测试任务注册
#[test]
fn test_job_registration() {
    let job = JobRegistration {
        id: "cleanup".to_string(),
        name: "Cleanup Task".to_string(),
        schedule: "0 0 * * *".to_string(),
        handler: "cleanup_handler".to_string(),
        enabled: true,
    };

    assert_eq!(job.id, "cleanup");
    assert_eq!(job.schedule, "0 0 * * *");
    assert!(job.enabled);
}

#[tokio::test]
async fn test_discover_async_with_timeout_missing_dir() {
    let missing = std::env::temp_dir().join("plog-plugin-missing-dir");
    let mut manager = PluginManager::new(&missing);
    let items = manager.discover_async_with_timeout().await.unwrap();
    assert!(items.is_empty());
}

#[tokio::test]
async fn test_discover_async_with_timeout_reads_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let plugin_dir = dir.path().join("demo-plugin");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("plugin.toml"),
        r#"
id = "demo-plugin"
name = "Demo Plugin"
version = "1.0.0"
"#,
    )
    .unwrap();

    let mut manager = PluginManager::new(dir.path());
    let items = manager.discover_async_with_timeout().await.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].manifest.id, "demo-plugin");
}
