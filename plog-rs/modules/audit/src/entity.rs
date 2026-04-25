//! Audit Entity

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "audit_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(nullable)]
    pub user_id: Option<i32>,
    pub action: String,
    pub resource_type: String,
    #[sea_orm(nullable)]
    pub resource_id: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub details: Option<String>,
    pub ip: String,
    pub user_agent: String,
    pub status: String,
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod actions {
    pub const LOGIN: &str = "login";
    pub const LOGOUT: &str = "logout";
    pub const CREATE: &str = "create";
    pub const UPDATE: &str = "update";
    pub const DELETE: &str = "delete";
}

pub mod statuses {
    pub const SUCCESS: &str = "success";
    pub const FAILED: &str = "failed";
}

#[derive(Debug, Clone)]
pub enum Action {
    Login,
    Logout,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
pub enum Status {
    Success,
    Failed,
}
