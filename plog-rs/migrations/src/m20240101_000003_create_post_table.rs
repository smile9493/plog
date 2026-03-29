//! 文章表迁移

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Gid))
                    .col(string(Post::Title).string_len(255).not_null())
                    .col(text(Post::Content).not_null())
                    .col(text_null(Post::Excerpt))
                    .col(integer(Post::Author).not_null())
                    .col(integer(Post::Sortid).not_null().default(0))
                    .col(big_integer(Post::Date).not_null())
                    .col(string(Post::Hide).string_len(1).not_null().default("n"))
                    .col(string(Post::Type).string_len(32).not_null().default("blog"))
                    .col(integer(Post::Views).not_null().default(0))
                    .col(integer(Post::Comnum).not_null().default(0))
                    .col(integer(Post::LikeCount).not_null().default(0))
                    .col(string(Post::Top).string_len(1).not_null().default("n"))
                    .col(string(Post::Sortop).string_len(1).not_null().default("n"))
                    .col(string(Post::AllowRemark).string_len(1).not_null().default("y"))
                    .col(string_null(Post::Password).string_len(255))
                    .col(string_null(Post::Cover).string_len(255))
                    .col(string_null(Post::Alias).string_len(255))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_author")
                            .from(Post::Table, Post::Author)
                            .to(User::Table, User::Uid)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Gid,
    Title,
    Content,
    Excerpt,
    Author,
    Sortid,
    Date,
    Hide,
    Type,
    Views,
    Comnum,
    LikeCount,
    Top,
    Sortop,
    AllowRemark,
    Password,
    Cover,
    Alias,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Uid,
}
