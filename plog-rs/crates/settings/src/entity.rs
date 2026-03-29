//! 设置实体定义

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 设置模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 设置键
    #[sea_orm(unique)]
    pub key: String,

    /// 设置值 (JSON)
    #[sea_orm(column_type = "Text")]
    pub value: String,

    /// 分组
    pub group: String,

    /// 描述
    #[sea_orm(nullable)]
    pub description: Option<String>,

    /// 是否系统设置
    pub is_system: bool,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 设置分组
pub mod groups {
    pub const GENERAL: &str = "general";
    pub const SITE: &str = "site";
    pub const SEO: &str = "seo";
    pub const MAIL: &str = "mail";
    pub const UPLOAD: &str = "upload";
    pub const COMMENT: &str = "comment";
    pub const CACHE: &str = "cache";
    pub const SECURITY: &str = "security";
}

/// 预定义设置键
pub mod keys {
    // General
    pub const SITE_NAME: &str = "site_name";
    pub const SITE_URL: &str = "site_url";
    pub const SITE_DESCRIPTION: &str = "site_description";
    pub const SITE_KEYWORDS: &str = "site_keywords";
    pub const SITE_LOGO: &str = "site_logo";
    pub const SITE_FAVICON: &str = "site_favicon";
    pub const SITE_LANGUAGE: &str = "site_language";
    pub const SITE_TIMEZONE: &str = "site_timezone";

    // SEO
    pub const SEO_TITLE: &str = "seo_title";
    pub const SEO_DESCRIPTION: &str = "seo_description";
    pub const SEO_KEYWORDS: &str = "seo_keywords";
    pub const SEO_ROBOTS: &str = "seo_robots";

    // Mail
    pub const MAIL_DRIVER: &str = "mail_driver";
    pub const MAIL_HOST: &str = "mail_host";
    pub const MAIL_PORT: &str = "mail_port";
    pub const MAIL_USERNAME: &str = "mail_username";
    pub const MAIL_PASSWORD: &str = "mail_password";
    pub const MAIL_FROM: &str = "mail_from";

    // Upload
    pub const UPLOAD_MAX_SIZE: &str = "upload_max_size";
    pub const UPLOAD_ALLOWED_TYPES: &str = "upload_allowed_types";
    pub const UPLOAD_PATH: &str = "upload_path";

    // Comment
    pub const COMMENT_ENABLED: &str = "comment_enabled";
    pub const COMMENT_MODERATION: &str = "comment_moderation";
    pub const COMMENT_SPAM_FILTER: &str = "comment_spam_filter";

    // Cache
    pub const CACHE_DRIVER: &str = "cache_driver";
    pub const CACHE_TTL: &str = "cache_ttl";

    // Security
    pub const SECURITY_LOGIN_ATTEMPTS: &str = "security_login_attempts";
    pub const SECURITY_LOCKOUT_TIME: &str = "security_lockout_time";
    pub const SECURITY_PASSWORD_MIN_LENGTH: &str = "security_password_min_length";
}

/// 默认设置值
pub fn default_settings() -> Vec<(&'static str, &'static str, serde_json::Value, &'static str)> {
    vec![
        // General
        (
            keys::SITE_NAME,
            groups::GENERAL,
            serde_json::json!("Plog CMS"),
            "站点名称",
        ),
        (
            keys::SITE_URL,
            groups::GENERAL,
            serde_json::json!("http://localhost"),
            "站点地址",
        ),
        (
            keys::SITE_DESCRIPTION,
            groups::GENERAL,
            serde_json::json!(""),
            "站点描述",
        ),
        (
            keys::SITE_KEYWORDS,
            groups::GENERAL,
            serde_json::json!(""),
            "站点关键词",
        ),
        (
            keys::SITE_LANGUAGE,
            groups::GENERAL,
            serde_json::json!("zh-CN"),
            "站点语言",
        ),
        (
            keys::SITE_TIMEZONE,
            groups::GENERAL,
            serde_json::json!("Asia/Shanghai"),
            "时区",
        ),
        // SEO
        (
            keys::SEO_TITLE,
            groups::SEO,
            serde_json::json!(""),
            "SEO 标题",
        ),
        (
            keys::SEO_DESCRIPTION,
            groups::SEO,
            serde_json::json!(""),
            "SEO 描述",
        ),
        (
            keys::SEO_KEYWORDS,
            groups::SEO,
            serde_json::json!(""),
            "SEO 关键词",
        ),
        (
            keys::SEO_ROBOTS,
            groups::SEO,
            serde_json::json!("index, follow"),
            "Robots",
        ),
        // Upload
        (
            keys::UPLOAD_MAX_SIZE,
            groups::UPLOAD,
            serde_json::json!(10485760),
            "最大上传大小 (bytes)",
        ),
        (
            keys::UPLOAD_ALLOWED_TYPES,
            groups::UPLOAD,
            serde_json::json!("jpg,jpeg,png,gif,webp,pdf,doc,docx,xls,xlsx,zip"),
            "允许的文件类型",
        ),
        (
            keys::UPLOAD_PATH,
            groups::UPLOAD,
            serde_json::json!("content/uploadfile"),
            "上传路径",
        ),
        // Comment
        (
            keys::COMMENT_ENABLED,
            groups::COMMENT,
            serde_json::json!(true),
            "启用评论",
        ),
        (
            keys::COMMENT_MODERATION,
            groups::COMMENT,
            serde_json::json!(true),
            "评论审核",
        ),
        (
            keys::COMMENT_SPAM_FILTER,
            groups::COMMENT,
            serde_json::json!(true),
            "垃圾评论过滤",
        ),
        // Cache
        (
            keys::CACHE_DRIVER,
            groups::CACHE,
            serde_json::json!("file"),
            "缓存驱动",
        ),
        (
            keys::CACHE_TTL,
            groups::CACHE,
            serde_json::json!(3600),
            "缓存过期时间 (秒)",
        ),
        // Security
        (
            keys::SECURITY_LOGIN_ATTEMPTS,
            groups::SECURITY,
            serde_json::json!(5),
            "登录尝试次数",
        ),
        (
            keys::SECURITY_LOCKOUT_TIME,
            groups::SECURITY,
            serde_json::json!(900),
            "锁定时间 (秒)",
        ),
        (
            keys::SECURITY_PASSWORD_MIN_LENGTH,
            groups::SECURITY,
            serde_json::json!(6),
            "密码最小长度",
        ),
    ]
}
