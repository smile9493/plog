//! 分类实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 分类模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sort")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub sid: i32,
    pub sortname: String,
    pub pid: i32,
    pub sortorder: i32,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    #[sea_orm(nullable)]
    pub alias: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
