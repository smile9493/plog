//! 评论实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 评论模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cid: i32,
    pub gid: i32,
    pub pid: i32,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub poster: String,
    pub email: String,
    pub url: String,
    pub ip: String,
    pub date: i64,
    pub hide: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
