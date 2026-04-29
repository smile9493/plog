//! Media Entity

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub filename: String,
    pub original_name: String,
    pub filepath: String,
    pub url: String,
    pub filesize: i64,
    pub mimetype: String,
    pub extension: String,
    #[sea_orm(nullable)]
    pub width: Option<i32>,
    #[sea_orm(nullable)]
    pub height: Option<i32>,
    #[sea_orm(nullable)]
    pub user_id: Option<i32>,
    pub storage_driver: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub fn get_extension(filename: &str) -> String {
    filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase()
}

pub fn generate_unique_filename(original: &str) -> String {
    let ext = get_extension(original);
    let timestamp = chrono::Utc::now().timestamp_millis();
    let uuid = uuid::Uuid::new_v4().to_string()[..8].to_string();
    format!("{}_{}.{}", timestamp, uuid, ext)
}

pub fn default_allowed_extensions() -> Vec<&'static str> {
    vec![
        "jpg", "jpeg", "png", "gif", "webp", "svg",
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
        "zip", "rar", "7z",
    ]
}
