//! Theme 单元测试

use plog_theme::*;

/// 测试主题管理器创建
#[test]
fn test_theme_manager_new() {
    let manager = ThemeManager::new();
    assert_eq!(manager.get_all_themes().len(), 0);
    assert!(manager.get_active_theme().is_none());
}

/// 测试加载主题
#[test]
fn test_load_theme() {
    let mut manager = ThemeManager::new();

    let info = types::ThemeInfo {
        name: "test-theme".to_string(),
        version: "1.0.0".to_string(),
        description: "Test theme".to_string(),
        author: "Test".to_string(),
        engine: types::ThemeEngine::Blade,
        templates: std::collections::HashMap::new(),
        assets: types::ThemeAssets {
            css: vec!["style.css".to_string()],
            js: vec!["app.js".to_string()],
            images: vec![],
        },
        supports: vec!["responsive".to_string()],
    };

    let result = manager.load_theme("test-theme", info);
    assert!(result.is_ok());
}

/// 测试重复加载主题
#[test]
fn test_load_duplicate_theme() {
    let mut manager = ThemeManager::new();

    let info = types::ThemeInfo {
        name: "test-theme".to_string(),
        version: "1.0.0".to_string(),
        description: "Test theme".to_string(),
        author: "Test".to_string(),
        engine: types::ThemeEngine::Blade,
        templates: std::collections::HashMap::new(),
        assets: types::ThemeAssets {
            css: vec![],
            js: vec![],
            images: vec![],
        },
        supports: vec![],
    };

    manager.load_theme("test-theme", info.clone()).unwrap();
    let result = manager.load_theme("test-theme", info);
    assert!(result.is_err());
}

/// 测试激活主题
#[test]
fn test_activate_theme() {
    let mut manager = ThemeManager::new();

    let info = types::ThemeInfo {
        name: "test-theme".to_string(),
        version: "1.0.0".to_string(),
        description: "Test theme".to_string(),
        author: "Test".to_string(),
        engine: types::ThemeEngine::Blade,
        templates: std::collections::HashMap::new(),
        assets: types::ThemeAssets {
            css: vec![],
            js: vec![],
            images: vec![],
        },
        supports: vec![],
    };

    manager.load_theme("test-theme", info).unwrap();
    let result = manager.activate_theme("test-theme");
    assert!(result.is_ok());

    let active = manager.get_active_theme();
    assert!(active.is_some());
    assert_eq!(active.unwrap().name, "test-theme");
}

/// 测试激活不存在的主题
#[test]
fn test_activate_nonexistent_theme() {
    let mut manager = ThemeManager::new();
    let result = manager.activate_theme("nonexistent");
    assert!(result.is_err());
}

/// 测试卸载主题
#[test]
fn test_unload_theme() {
    let mut manager = ThemeManager::new();

    let info = types::ThemeInfo {
        name: "test-theme".to_string(),
        version: "1.0.0".to_string(),
        description: "Test theme".to_string(),
        author: "Test".to_string(),
        engine: types::ThemeEngine::Blade,
        templates: std::collections::HashMap::new(),
        assets: types::ThemeAssets {
            css: vec![],
            js: vec![],
            images: vec![],
        },
        supports: vec![],
    };

    manager.load_theme("test-theme", info).unwrap();
    let result = manager.unload_theme("test-theme");
    assert!(result.is_ok());
}

/// 测试卸载激活中的主题
#[test]
fn test_unload_active_theme() {
    let mut manager = ThemeManager::new();

    let info = types::ThemeInfo {
        name: "test-theme".to_string(),
        version: "1.0.0".to_string(),
        description: "Test theme".to_string(),
        author: "Test".to_string(),
        engine: types::ThemeEngine::Blade,
        templates: std::collections::HashMap::new(),
        assets: types::ThemeAssets {
            css: vec![],
            js: vec![],
            images: vec![],
        },
        supports: vec![],
    };

    manager.load_theme("test-theme", info).unwrap();
    manager.activate_theme("test-theme").unwrap();

    // 应该失败，不能卸载激活中的主题
    let result = manager.unload_theme("test-theme");
    assert!(result.is_err());
}

/// 测试获取所有主题
#[test]
fn test_get_all_themes() {
    let mut manager = ThemeManager::new();

    for i in 1..=3 {
        let info = types::ThemeInfo {
            name: format!("theme-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Theme {}", i),
            author: "Test".to_string(),
            engine: types::ThemeEngine::Blade,
            templates: std::collections::HashMap::new(),
            assets: types::ThemeAssets {
                css: vec![],
                js: vec![],
                images: vec![],
            },
            supports: vec![],
        };
        manager.load_theme(&format!("theme-{}", i), info).unwrap();
    }

    assert_eq!(manager.get_all_themes().len(), 3);
}

/// 测试主题引擎类型
#[test]
fn test_theme_engine() {
    use types::ThemeEngine;

    let blade = ThemeEngine::Blade;
    let twig = ThemeEngine::Twig;
    let custom = ThemeEngine::Custom("vue".to_string());

    assert_ne!(blade, twig);
    assert_ne!(blade, custom);
}
