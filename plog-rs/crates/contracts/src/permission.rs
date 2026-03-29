//! 权限模型定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 角色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub is_system: bool,
}

/// 权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub group: String,
}

/// 权限组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroup {
    pub name: String,
    pub slug: String,
    pub permissions: Vec<Permission>,
}

/// 系统预定义角色
pub mod roles {
    pub const ADMIN: &str = "admin";
    pub const EDITOR: &str = "editor";
    pub const USER: &str = "user";
}

/// 系统预定义权限
pub mod permissions {
    // 用户管理
    pub const USER_VIEW: &str = "user:view";
    pub const USER_CREATE: &str = "user:create";
    pub const USER_UPDATE: &str = "user:update";
    pub const USER_DELETE: &str = "user:delete";

    // 文章管理
    pub const POST_VIEW: &str = "post:view";
    pub const POST_CREATE: &str = "post:create";
    pub const POST_UPDATE: &str = "post:update";
    pub const POST_DELETE: &str = "post:delete";
    pub const POST_PUBLISH: &str = "post:publish";

    // 分类管理
    pub const CATEGORY_VIEW: &str = "category:view";
    pub const CATEGORY_CREATE: &str = "category:create";
    pub const CATEGORY_UPDATE: &str = "category:update";
    pub const CATEGORY_DELETE: &str = "category:delete";

    // 标签管理
    pub const TAG_VIEW: &str = "tag:view";
    pub const TAG_CREATE: &str = "tag:create";
    pub const TAG_UPDATE: &str = "tag:update";
    pub const TAG_DELETE: &str = "tag:delete";

    // 评论管理
    pub const COMMENT_VIEW: &str = "comment:view";
    pub const COMMENT_APPROVE: &str = "comment:approve";
    pub const COMMENT_DELETE: &str = "comment:delete";

    // 媒体管理
    pub const MEDIA_VIEW: &str = "media:view";
    pub const MEDIA_UPLOAD: &str = "media:upload";
    pub const MEDIA_DELETE: &str = "media:delete";

    // 插件管理
    pub const PLUGIN_VIEW: &str = "plugin:view";
    pub const PLUGIN_INSTALL: &str = "plugin:install";
    pub const PLUGIN_UNINSTALL: &str = "plugin:uninstall";
    pub const PLUGIN_CONFIGURE: &str = "plugin:configure";

    // 主题管理
    pub const THEME_VIEW: &str = "theme:view";
    pub const THEME_INSTALL: &str = "theme:install";
    pub const THEME_ACTIVATE: &str = "theme:activate";
    pub const THEME_CONFIGURE: &str = "theme:configure";

    // 系统设置
    pub const SETTINGS_VIEW: &str = "settings:view";
    pub const SETTINGS_UPDATE: &str = "settings:update";

    // 审计日志
    pub const AUDIT_VIEW: &str = "audit:view";
}

/// 默认角色权限映射
pub fn default_role_permissions() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // Admin 拥有所有权限
    map.insert(
        roles::ADMIN,
        vec![
            permissions::USER_VIEW,
            permissions::USER_CREATE,
            permissions::USER_UPDATE,
            permissions::USER_DELETE,
            permissions::POST_VIEW,
            permissions::POST_CREATE,
            permissions::POST_UPDATE,
            permissions::POST_DELETE,
            permissions::POST_PUBLISH,
            permissions::CATEGORY_VIEW,
            permissions::CATEGORY_CREATE,
            permissions::CATEGORY_UPDATE,
            permissions::CATEGORY_DELETE,
            permissions::TAG_VIEW,
            permissions::TAG_CREATE,
            permissions::TAG_UPDATE,
            permissions::TAG_DELETE,
            permissions::COMMENT_VIEW,
            permissions::COMMENT_APPROVE,
            permissions::COMMENT_DELETE,
            permissions::MEDIA_VIEW,
            permissions::MEDIA_UPLOAD,
            permissions::MEDIA_DELETE,
            permissions::PLUGIN_VIEW,
            permissions::PLUGIN_INSTALL,
            permissions::PLUGIN_UNINSTALL,
            permissions::PLUGIN_CONFIGURE,
            permissions::THEME_VIEW,
            permissions::THEME_INSTALL,
            permissions::THEME_ACTIVATE,
            permissions::THEME_CONFIGURE,
            permissions::SETTINGS_VIEW,
            permissions::SETTINGS_UPDATE,
            permissions::AUDIT_VIEW,
        ],
    );

    // Editor 可以管理内容
    map.insert(
        roles::EDITOR,
        vec![
            permissions::POST_VIEW,
            permissions::POST_CREATE,
            permissions::POST_UPDATE,
            permissions::POST_DELETE,
            permissions::CATEGORY_VIEW,
            permissions::CATEGORY_CREATE,
            permissions::CATEGORY_UPDATE,
            permissions::TAG_VIEW,
            permissions::TAG_CREATE,
            permissions::TAG_UPDATE,
            permissions::COMMENT_VIEW,
            permissions::COMMENT_APPROVE,
            permissions::MEDIA_VIEW,
            permissions::MEDIA_UPLOAD,
        ],
    );

    // User 只能查看
    map.insert(
        roles::USER,
        vec![
            permissions::POST_VIEW,
            permissions::CATEGORY_VIEW,
            permissions::TAG_VIEW,
            permissions::COMMENT_VIEW,
            permissions::MEDIA_VIEW,
        ],
    );

    map
}
