//! Plugin 单元测试

use plog_plugin::*;

/// 测试插件管理器创建
#[test]
fn test_plugin_manager_new() {
    let manager = PluginManager::new();
    assert_eq!(manager.get_all_plugins().len(), 0);
}

/// 测试加载插件
#[test]
fn test_load_plugin() {
    let mut manager = PluginManager::new();

    let info = types::PluginInfo {
        name: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "Test plugin".to_string(),
        author: "Test".to_string(),
        status: types::PluginStatus::Active,
        capabilities: vec!["admin".to_string()],
        hooks: vec!["post_save".to_string()],
    };

    let result = manager.load_plugin("test-plugin", info);
    assert!(result.is_ok());
    assert!(manager.has_plugin("test-plugin"));
}

/// 测试重复加载插件
#[test]
fn test_load_duplicate_plugin() {
    let mut manager = PluginManager::new();

    let info = types::PluginInfo {
        name: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "Test plugin".to_string(),
        author: "Test".to_string(),
        status: types::PluginStatus::Active,
        capabilities: vec![],
        hooks: vec![],
    };

    manager.load_plugin("test-plugin", info.clone()).unwrap();
    let result = manager.load_plugin("test-plugin", info);
    assert!(result.is_err());
}

/// 测试卸载插件
#[test]
fn test_unload_plugin() {
    let mut manager = PluginManager::new();

    let info = types::PluginInfo {
        name: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "Test plugin".to_string(),
        author: "Test".to_string(),
        status: types::PluginStatus::Active,
        capabilities: vec![],
        hooks: vec![],
    };

    manager.load_plugin("test-plugin", info).unwrap();
    assert!(manager.has_plugin("test-plugin"));

    let result = manager.unload_plugin("test-plugin");
    assert!(result.is_ok());
    assert!(!manager.has_plugin("test-plugin"));
}

/// 测试卸载不存在的插件
#[test]
fn test_unload_nonexistent_plugin() {
    let mut manager = PluginManager::new();
    let result = manager.unload_plugin("nonexistent");
    assert!(result.is_err());
}

/// 测试获取插件信息
#[test]
fn test_get_plugin() {
    let mut manager = PluginManager::new();

    let info = types::PluginInfo {
        name: "test-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "Test plugin".to_string(),
        author: "Test".to_string(),
        status: types::PluginStatus::Active,
        capabilities: vec!["admin".to_string()],
        hooks: vec![],
    };

    manager.load_plugin("test-plugin", info).unwrap();

    let plugin = manager.get_plugin("test-plugin");
    assert!(plugin.is_some());
    assert_eq!(plugin.unwrap().version, "1.0.0");
}

/// 测试获取所有插件
#[test]
fn test_get_all_plugins() {
    let mut manager = PluginManager::new();

    for i in 1..=3 {
        let info = types::PluginInfo {
            name: format!("plugin-{}", i),
            version: "1.0.0".to_string(),
            description: format!("Plugin {}", i),
            author: "Test".to_string(),
            status: types::PluginStatus::Active,
            capabilities: vec![],
            hooks: vec![],
        };
        manager.load_plugin(&format!("plugin-{}", i), info).unwrap();
    }

    assert_eq!(manager.get_all_plugins().len(), 3);
}

/// 测试插件状态
#[test]
fn test_plugin_status() {
    use types::PluginStatus;

    assert_eq!(PluginStatus::Active, PluginStatus::Active);
    assert_ne!(PluginStatus::Active, PluginStatus::Inactive);
}
