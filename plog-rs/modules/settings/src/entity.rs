//! Settings Entity

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub key: String,
    pub value: String,
    pub group: String,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod keys {
    pub const SITE_NAME: &str = "site_name";
    pub const SITE_URL: &str = "site_url";
    pub const SITE_DESCRIPTION: &str = "site_description";
    pub const COMMENT_ENABLED: &str = "comment_enabled";
    pub const UPLOAD_MAX_SIZE: &str = "upload_max_size";
}

pub fn default_settings() -> Vec<(&'static str, &'static str, serde_json::Value, &'static str)> {
    vec![
        (keys::SITE_NAME, "general", serde_json::json!("Plog CMS"), "Site name"),
        (keys::SITE_URL, "general", serde_json::json!("http://localhost"), "Site URL"),
        (keys::SITE_DESCRIPTION, "general", serde_json::json!("A simple CMS"), "Site description"),
        (keys::COMMENT_ENABLED, "discussion", serde_json::json!(true), "Enable comments"),
        (keys::UPLOAD_MAX_SIZE, "upload", serde_json::json!(10485760i64), "Max upload size in bytes"),
    ]
}
