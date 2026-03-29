//! 用户实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 用户模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: i32,
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub role: String,
    #[sea_orm(nullable)]
    pub email: Option<String>,
    #[sea_orm(nullable)]
    pub photo: Option<String>,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    pub create_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
