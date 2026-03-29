//! 文章实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 文章模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub gid: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub excerpt: Option<String>,
    pub author: i32,
    pub sortid: i32,
    pub date: i64,
    pub hide: String,
    pub r#type: String,
    pub views: i32,
    pub comnum: i32,
    pub like_count: i32,
    pub top: String,
    pub sortop: String,
    pub allow_remark: String,
    #[sea_orm(nullable)]
    pub password: Option<String>,
    #[sea_orm(nullable)]
    pub cover: Option<String>,
    #[sea_orm(nullable)]
    pub alias: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
