//! Theme 单元测试

use plog_theme::*;
use std::path::Path;

/// 测试主题管理器创建
#[test]
fn test_theme_manager_new() {
    let manager = ThemeManager::new("/tmp/themes");
    assert_eq!(manager.count(), 0);
    assert!(manager.get_active_theme().is_none());
}

/// 测试主题状态
#[test]
fn test_theme_status() {
    assert_eq!(ThemeStatus::Active, ThemeStatus::Active);
    assert_ne!(ThemeStatus::Active, ThemeStatus::Installed);
    assert_ne!(ThemeStatus::Installed, ThemeStatus::NotInstalled);
}

/// 测试主题 manifest 解析
#[test]
fn test_manifest_parse() {
    let toml_str = r#"
id = "test-theme"
name = "Test Theme"
version = "1.0.0"
description = "A test theme"
author = "Test Author"
license = "MIT"

[[layouts]]
id = "default"
name = "Default Layout"
template = "layout.html"
default = true

[[slots]]
id = "header"
name = "Header"

[[slots]]
id = "footer"
name = "Footer"

[[page_templates]]
id = "post"
name = "Post Template"
template = "post.html"
page_type = "post"
"#;

    let manifest: ThemeManifest = toml::from_str(toml_str).unwrap();

    assert_eq!(manifest.id, "test-theme");
    assert_eq!(manifest.name, "Test Theme");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.layouts.len(), 1);
    assert_eq!(manifest.slots.len(), 2);
    assert_eq!(manifest.page_templates.len(), 1);
}

/// 测试主题信息创建
#[test]
fn test_theme_info_from_manifest() {
    let manifest = ThemeManifest {
        id: "test-theme".to_string(),
        name: "Test Theme".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test".to_string()),
        author: Some("Test".to_string()),
        homepage: None,
        license: Some("MIT".to_string()),
        requires: None,
        preview: None,
        layouts: vec![],
        slots: vec![],
        page_templates: vec![],
        settings_schema: None,
        settings_defaults: None,
        assets: None,
        supported_features: vec![],
    };

    let info = ThemeInfo::from_manifest(manifest, "/tmp/themes/test-theme");

    assert_eq!(info.manifest.id, "test-theme");
    assert_eq!(info.status, ThemeStatus::Installed);
    assert!(info.installed_at.is_some());
}

/// 测试布局获取
#[test]
fn test_get_layout() {
    let manifest = ThemeManifest {
        id: "test".to_string(),
        name: "Test".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        author: None,
        homepage: None,
        license: None,
        requires: None,
        preview: None,
        layouts: vec![
            LayoutDefinition {
                id: "default".to_string(),
                name: "Default".to_string(),
                template: "layout.html".to_string(),
                default: true,
            },
            LayoutDefinition {
                id: "sidebar".to_string(),
                name: "Sidebar".to_string(),
                template: "sidebar.html".to_string(),
                default: false,
            },
        ],
        slots: vec![],
        page_templates: vec![],
        settings_schema: None,
        settings_defaults: None,
        assets: None,
        supported_features: vec![],
    };

    let info = ThemeInfo::from_manifest(manifest, "");

    assert!(info.get_layout("default").is_some());
    assert!(info.get_layout("sidebar").is_some());
    assert!(info.get_layout("nonexistent").is_none());
    assert!(info.get_default_layout().is_some());
}

/// 测试页面模板获取
#[test]
fn test_get_page_templates() {
    let manifest = ThemeManifest {
        id: "test".to_string(),
        name: "Test".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        author: None,
        homepage: None,
        license: None,
        requires: None,
        preview: None,
        layouts: vec![],
        slots: vec![],
        page_templates: vec![PageTemplate {
            id: "post".to_string(),
            name: "Post".to_string(),
            template: "post.html".to_string(),
            page_type: Some("post".to_string()),
        }],
        settings_schema: None,
        settings_defaults: None,
        assets: None,
        supported_features: vec![],
    };

    let info = ThemeInfo::from_manifest(manifest, "");

    assert_eq!(info.get_page_templates().len(), 1);
    assert_eq!(info.get_page_templates()[0].id, "post");
}

/// 测试特性支持
#[test]
fn test_supports_feature() {
    let manifest = ThemeManifest {
        id: "test".to_string(),
        name: "Test".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        author: None,
        homepage: None,
        license: None,
        requires: None,
        preview: None,
        layouts: vec![],
        slots: vec![],
        page_templates: vec![],
        settings_schema: None,
        settings_defaults: None,
        assets: None,
        supported_features: vec![ThemeFeature::Responsive, ThemeFeature::DarkMode],
    };

    let info = ThemeInfo::from_manifest(manifest, "");

    assert!(info.supports_feature(&ThemeFeature::Responsive));
    assert!(info.supports_feature(&ThemeFeature::DarkMode));
    assert!(!info.supports_feature(&ThemeFeature::Search));
}

/// 测试布局定义
#[test]
fn test_layout_definition() {
    let layout = LayoutDefinition {
        id: "default".to_string(),
        name: "Default Layout".to_string(),
        template: "layout.html".to_string(),
        default: true,
    };

    assert_eq!(layout.id, "default");
    assert_eq!(layout.name, "Default Layout");
    assert!(layout.default);
}

/// 测试插槽定义
#[test]
fn test_slot_definition() {
    let slot = SlotDefinition {
        id: "header".to_string(),
        name: "Header".to_string(),
        description: Some("Page header".to_string()),
        default: Some("<header>Default</header>".to_string()),
    };

    assert_eq!(slot.id, "header");
    assert_eq!(slot.name, "Header");
    assert!(slot.default.is_some());
}

/// 测试页面模板
#[test]
fn test_page_template() {
    let template = PageTemplate {
        id: "post".to_string(),
        name: "Post Template".to_string(),
        template: "post.html".to_string(),
        page_type: Some("post".to_string()),
    };

    assert_eq!(template.id, "post");
    assert_eq!(template.template, "post.html");
}
