//! 分类表迁移

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(pk_auto(Category::Sid))
                    .col(string(Category::Sortname).string_len(64).not_null())
                    .col(integer(Category::Pid).not_null().default(0))
                    .col(integer(Category::Sortorder).not_null().default(0))
                    .col(string_null(Category::Description).string_len(255))
                    .col(string_null(Category::Alias).string_len(64))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Sid,
    Sortname,
    Pid,
    Sortorder,
    Description,
    Alias,
}
