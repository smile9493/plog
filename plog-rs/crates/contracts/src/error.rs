//! 错误码规范定义

use serde::{Deserialize, Serialize};

/// API 错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// 错误码
    pub code: String,

    /// 错误消息
    pub message: String,

    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// 标准错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // 4xx 客户端错误
    /// 400 请求参数错误
    BadRequest,
    /// 401 未认证
    Unauthorized,
    /// 403 无权限
    Forbidden,
    /// 404 资源不存在
    NotFound,
    /// 405 方法不允许
    MethodNotAllowed,
    /// 409 资源冲突
    Conflict,
    /// 422 验证失败
    ValidationError,
    /// 429 请求过多
    TooManyRequests,

    // 5xx 服务端错误
    /// 500 内部错误
    InternalError,
    /// 501 未实现
    NotImplemented,
    /// 503 服务不可用
    ServiceUnavailable,

    // 业务错误
    /// 认证失败
    AuthFailed,
    /// Token 过期
    TokenExpired,
    /// Token 无效
    TokenInvalid,
    /// 密码错误
    PasswordIncorrect,
    /// 用户已存在
    UserAlreadyExists,
    /// 用户不存在
    UserNotFound,
    /// 角色不存在
    RoleNotFound,
    /// 权限不足
    PermissionDenied,

    /// 文章不存在
    PostNotFound,
    /// 分类不存在
    CategoryNotFound,
    /// 标签不存在
    TagNotFound,
    /// 评论不存在
    CommentNotFound,

    /// 插件不存在
    PluginNotFound,
    /// 插件已安装
    PluginAlreadyInstalled,
    /// 插件安装失败
    PluginInstallFailed,
    /// 插件卸载失败
    PluginUninstallFailed,

    /// 主题不存在
    ThemeNotFound,
    /// 主题已安装
    ThemeAlreadyInstalled,
    /// 主题安装失败
    ThemeInstallFailed,

    /// 文件上传失败
    FileUploadFailed,
    /// 文件过大
    FileTooLarge,
    /// 文件类型不允许
    FileTypeNotAllowed,

    /// 数据库错误
    DatabaseError,
    /// 缓存错误
    CacheError,
}

impl ErrorCode {
    /// 获取错误码字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            // 4xx
            Self::BadRequest => "BAD_REQUEST",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::MethodNotAllowed => "METHOD_NOT_ALLOWED",
            Self::Conflict => "CONFLICT",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::TooManyRequests => "TOO_MANY_REQUESTS",

            // 5xx
            Self::InternalError => "INTERNAL_ERROR",
            Self::NotImplemented => "NOT_IMPLEMENTED",
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",

            // 业务
            Self::AuthFailed => "AUTH_FAILED",
            Self::TokenExpired => "TOKEN_EXPIRED",
            Self::TokenInvalid => "TOKEN_INVALID",
            Self::PasswordIncorrect => "PASSWORD_INCORRECT",
            Self::UserAlreadyExists => "USER_ALREADY_EXISTS",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::RoleNotFound => "ROLE_NOT_FOUND",
            Self::PermissionDenied => "PERMISSION_DENIED",

            Self::PostNotFound => "POST_NOT_FOUND",
            Self::CategoryNotFound => "CATEGORY_NOT_FOUND",
            Self::TagNotFound => "TAG_NOT_FOUND",
            Self::CommentNotFound => "COMMENT_NOT_FOUND",

            Self::PluginNotFound => "PLUGIN_NOT_FOUND",
            Self::PluginAlreadyInstalled => "PLUGIN_ALREADY_INSTALLED",
            Self::PluginInstallFailed => "PLUGIN_INSTALL_FAILED",
            Self::PluginUninstallFailed => "PLUGIN_UNINSTALL_FAILED",

            Self::ThemeNotFound => "THEME_NOT_FOUND",
            Self::ThemeAlreadyInstalled => "THEME_ALREADY_INSTALLED",
            Self::ThemeInstallFailed => "THEME_INSTALL_FAILED",

            Self::FileUploadFailed => "FILE_UPLOAD_FAILED",
            Self::FileTooLarge => "FILE_TOO_LARGE",
            Self::FileTypeNotAllowed => "FILE_TYPE_NOT_ALLOWED",

            Self::DatabaseError => "DATABASE_ERROR",
            Self::CacheError => "CACHE_ERROR",
        }
    }

    /// 获取 HTTP 状态码
    pub fn http_status(&self) -> u16 {
        match self {
            // 4xx
            Self::BadRequest | Self::ValidationError => 400,
            Self::Unauthorized
            | Self::AuthFailed
            | Self::TokenExpired
            | Self::TokenInvalid
            | Self::PasswordIncorrect => 401,
            Self::Forbidden | Self::PermissionDenied => 403,
            Self::NotFound
            | Self::UserNotFound
            | Self::PostNotFound
            | Self::CategoryNotFound
            | Self::TagNotFound
            | Self::CommentNotFound
            | Self::PluginNotFound
            | Self::ThemeNotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::Conflict
            | Self::UserAlreadyExists
            | Self::PluginAlreadyInstalled
            | Self::ThemeAlreadyInstalled => 409,
            Self::TooManyRequests => 429,

            // 5xx
            Self::InternalError | Self::DatabaseError | Self::CacheError => 500,
            Self::NotImplemented => 501,
            Self::ServiceUnavailable => 503,

            // 业务错误默认 500
            _ => 500,
        }
    }

    /// 获取默认消息
    pub fn default_message(&self) -> &'static str {
        match self {
            Self::BadRequest => "请求参数错误",
            Self::Unauthorized => "未认证",
            Self::Forbidden => "无权限",
            Self::NotFound => "资源不存在",
            Self::MethodNotAllowed => "方法不允许",
            Self::Conflict => "资源冲突",
            Self::ValidationError => "验证失败",
            Self::TooManyRequests => "请求过多",

            Self::InternalError => "内部错误",
            Self::NotImplemented => "未实现",
            Self::ServiceUnavailable => "服务不可用",

            Self::AuthFailed => "认证失败",
            Self::TokenExpired => "Token 已过期",
            Self::TokenInvalid => "Token 无效",
            Self::PasswordIncorrect => "密码错误",
            Self::UserAlreadyExists => "用户已存在",
            Self::UserNotFound => "用户不存在",
            Self::RoleNotFound => "角色不存在",
            Self::PermissionDenied => "权限不足",

            Self::PostNotFound => "文章不存在",
            Self::CategoryNotFound => "分类不存在",
            Self::TagNotFound => "标签不存在",
            Self::CommentNotFound => "评论不存在",

            Self::PluginNotFound => "插件不存在",
            Self::PluginAlreadyInstalled => "插件已安装",
            Self::PluginInstallFailed => "插件安装失败",
            Self::PluginUninstallFailed => "插件卸载失败",

            Self::ThemeNotFound => "主题不存在",
            Self::ThemeAlreadyInstalled => "主题已安装",
            Self::ThemeInstallFailed => "主题安装失败",

            Self::FileUploadFailed => "文件上传失败",
            Self::FileTooLarge => "文件过大",
            Self::FileTypeNotAllowed => "文件类型不允许",

            Self::DatabaseError => "数据库错误",
            Self::CacheError => "缓存错误",
        }
    }
}
