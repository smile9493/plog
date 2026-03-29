//! 审计日志实体定义

use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

/// 审计日志模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 用户 ID
    #[sea_orm(nullable)]
    pub user_id: Option<i32>,

    /// 用户名
    #[sea_orm(nullable)]
    pub username: Option<String>,

    /// 操作类型
    pub action: String,

    /// 资源类型
    pub resource_type: String,

    /// 资源 ID
    #[sea_orm(nullable)]
    pub resource_id: Option<String>,

    /// 操作描述
    pub description: String,

    /// 变更前数据
    #[sea_orm(column_type = "Text", nullable)]
    pub old_value: Option<String>,

    /// 变更后数据
    #[sea_orm(column_type = "Text", nullable)]
    pub new_value: Option<String>,

    /// IP 地址
    #[sea_orm(nullable)]
    pub ip_address: Option<String>,

    /// User Agent
    #[sea_orm(nullable)]
    pub user_agent: Option<String>,

    /// 请求路径
    #[sea_orm(nullable)]
    pub request_path: Option<String>,

    /// 请求方法
    #[sea_orm(nullable)]
    pub request_method: Option<String>,

    /// 状态
    pub status: String,

    /// 错误信息
    #[sea_orm(nullable)]
    pub error_message: Option<String>,

    /// 创建时间
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 操作类型
pub mod actions {
    pub const CREATE: &str = "create";
    pub const READ: &str = "read";
    pub const UPDATE: &str = "update";
    pub const DELETE: &str = "delete";
    pub const LOGIN: &str = "login";
    pub const LOGOUT: &str = "logout";
    pub const EXPORT: &str = "export";
    pub const IMPORT: &str = "import";
    pub const APPROVE: &str = "approve";
    pub const REJECT: &str = "reject";
}

/// 资源类型
pub mod resources {
    pub const USER: &str = "user";
    pub const POST: &str = "post";
    pub const CATEGORY: &str = "category";
    pub const TAG: &str = "tag";
    pub const COMMENT: &str = "comment";
    pub const MEDIA: &str = "media";
    pub const SETTING: &str = "setting";
    pub const PLUGIN: &str = "plugin";
    pub const THEME: &str = "theme";
}

/// 日志状态
pub mod statuses {
    pub const SUCCESS: &str = "success";
    pub const FAILED: &str = "failed";
    pub const PENDING: &str = "pending";
}

/// 审计日志构建器
pub struct AuditLogBuilder {
    user_id: Option<i32>,
    username: Option<String>,
    action: String,
    resource_type: String,
    resource_id: Option<String>,
    description: String,
    old_value: Option<String>,
    new_value: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    request_path: Option<String>,
    request_method: Option<String>,
    status: String,
    error_message: Option<String>,
}

impl AuditLogBuilder {
    pub fn new(action: &str, resource_type: &str) -> Self {
        Self {
            user_id: None,
            username: None,
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: None,
            description: String::new(),
            old_value: None,
            new_value: None,
            ip_address: None,
            user_agent: None,
            request_path: None,
            request_method: None,
            status: statuses::SUCCESS.to_string(),
            error_message: None,
        }
    }

    pub fn user(mut self, user_id: i32, username: &str) -> Self {
        self.user_id = Some(user_id);
        self.username = Some(username.to_string());
        self
    }

    pub fn resource_id(mut self, id: impl ToString) -> Self {
        self.resource_id = Some(id.to_string());
        self
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn old_value(mut self, value: serde_json::Value) -> Self {
        self.old_value = Some(serde_json::to_string(&value).unwrap_or_default());
        self
    }

    pub fn new_value(mut self, value: serde_json::Value) -> Self {
        self.new_value = Some(serde_json::to_string(&value).unwrap_or_default());
        self
    }

    pub fn ip_address(mut self, ip: &str) -> Self {
        self.ip_address = Some(ip.to_string());
        self
    }

    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = Some(ua.to_string());
        self
    }

    pub fn request(mut self, method: &str, path: &str) -> Self {
        self.request_method = Some(method.to_string());
        self.request_path = Some(path.to_string());
        self
    }

    pub fn failed(mut self, error: &str) -> Self {
        self.status = statuses::FAILED.to_string();
        self.error_message = Some(error.to_string());
        self
    }

    pub fn build(self) -> ActiveModel {
        let now = chrono::Utc::now().timestamp();
        ActiveModel {
            user_id: Set(self.user_id),
            username: Set(self.username),
            action: Set(self.action),
            resource_type: Set(self.resource_type),
            resource_id: Set(self.resource_id),
            description: Set(self.description),
            old_value: Set(self.old_value),
            new_value: Set(self.new_value),
            ip_address: Set(self.ip_address),
            user_agent: Set(self.user_agent),
            request_path: Set(self.request_path),
            request_method: Set(self.request_method),
            status: Set(self.status),
            error_message: Set(self.error_message),
            created_at: Set(now),
            ..Default::default()
        }
    }
}
