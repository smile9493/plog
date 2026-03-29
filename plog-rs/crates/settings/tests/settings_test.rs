//! Settings 单元测试

use plog_settings::*;

/// 测试设置分组常量
#[test]
fn test_settings_groups() {
    use entity::groups;

    assert_eq!(groups::GENERAL, "general");
    assert_eq!(groups::SITE, "site");
    assert_eq!(groups::SEO, "seo");
    assert_eq!(groups::MAIL, "mail");
    assert_eq!(groups::UPLOAD, "upload");
    assert_eq!(groups::COMMENT, "comment");
    assert_eq!(groups::CACHE, "cache");
    assert_eq!(groups::SECURITY, "security");
}

/// 测试设置键常量
#[test]
fn test_settings_keys() {
    use entity::keys;

    assert_eq!(keys::SITE_NAME, "site_name");
    assert_eq!(keys::SITE_URL, "site_url");
    assert_eq!(keys::UPLOAD_MAX_SIZE, "upload_max_size");
    assert_eq!(keys::COMMENT_ENABLED, "comment_enabled");
    assert_eq!(keys::CACHE_DRIVER, "cache_driver");
}

/// 测试默认设置
#[test]
fn test_default_settings() {
    let defaults = entity::default_settings();

    assert!(!defaults.is_empty());

    // 检查站点名称默认值
    let site_name = defaults
        .iter()
        .find(|(key, _, _, _)| *key == entity::keys::SITE_NAME);
    assert!(site_name.is_some());

    let (_, group, value, _) = site_name.unwrap();
    assert_eq!(*group, entity::groups::GENERAL);
    assert_eq!(value.as_str().unwrap(), "Plog CMS");
}

/// 测试默认设置完整性
#[test]
fn test_default_settings_completeness() {
    use entity::{default_settings, groups, keys};

    let defaults = default_settings();

    // 检查所有预定义键都有默认值
    let defined_keys: Vec<&str> = defaults.iter().map(|(k, _, _, _)| *k).collect();

    // General
    assert!(defined_keys.contains(&keys::SITE_NAME));
    assert!(defined_keys.contains(&keys::SITE_URL));
    assert!(defined_keys.contains(&keys::SITE_LANGUAGE));

    // SEO
    assert!(defined_keys.contains(&keys::SEO_TITLE));
    assert!(defined_keys.contains(&keys::SEO_ROBOTS));

    // Upload
    assert!(defined_keys.contains(&keys::UPLOAD_MAX_SIZE));
    assert!(defined_keys.contains(&keys::UPLOAD_PATH));

    // Comment
    assert!(defined_keys.contains(&keys::COMMENT_ENABLED));

    // Cache
    assert!(defined_keys.contains(&keys::CACHE_DRIVER));
    assert!(defined_keys.contains(&keys::CACHE_TTL));

    // Security
    assert!(defined_keys.contains(&keys::SECURITY_LOGIN_ATTEMPTS));
    assert!(defined_keys.contains(&keys::SECURITY_PASSWORD_MIN_LENGTH));
}

/// 测试设置分组归属
#[test]
fn test_settings_group_assignment() {
    use entity::{default_settings, groups, keys};

    let defaults = default_settings();

    // 检查站点名称在 general 组
    let site_name = defaults
        .iter()
        .find(|(k, _, _, _)| *k == keys::SITE_NAME)
        .unwrap();
    assert_eq!(site_name.1, groups::GENERAL);

    // 检查 SEO 设置在 seo 组
    let seo_title = defaults
        .iter()
        .find(|(k, _, _, _)| *k == keys::SEO_TITLE)
        .unwrap();
    assert_eq!(seo_title.1, groups::SEO);

    // 检查上传设置在 upload 组
    let upload_max = defaults
        .iter()
        .find(|(k, _, _, _)| *k == keys::UPLOAD_MAX_SIZE)
        .unwrap();
    assert_eq!(upload_max.1, groups::UPLOAD);

    // 检查评论设置在 comment 组
    let comment_enabled = defaults
        .iter()
        .find(|(k, _, _, _)| *k == keys::COMMENT_ENABLED)
        .unwrap();
    assert_eq!(comment_enabled.1, groups::COMMENT);

    // 检查缓存设置在 cache 组
    let cache_driver = defaults
        .iter()
        .find(|(k, _, _, _)| *k == keys::CACHE_DRIVER)
        .unwrap();
    assert_eq!(cache_driver.1, groups::CACHE);

    // 检查安全设置在 security 组
    let login_attempts = defaults
        .iter()
        .find(|(k, _, _, _)| *k == keys::SECURITY_LOGIN_ATTEMPTS)
        .unwrap();
    assert_eq!(login_attempts.1, groups::SECURITY);
}
