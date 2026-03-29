//! 评论表迁移

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(pk_auto(Comment::Cid))
                    .col(integer(Comment::Gid).not_null())
                    .col(integer(Comment::Pid).not_null().default(0))
                    .col(text(Comment::Content).not_null())
                    .col(string(Comment::Poster).string_len(64).not_null())
                    .col(string(Comment::Email).string_len(100).not_null())
                    .col(string(Comment::Url).string_len(255).not_null().default(""))
                    .col(string(Comment::Ip).string_len(64).not_null())
                    .col(big_integer(Comment::Date).not_null())
                    .col(string(Comment::Hide).string_len(1).not_null().default("n"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_post")
                            .from(Comment::Table, Comment::Gid)
                            .to(Post::Table, Post::Gid)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Cid,
    Gid,
    Pid,
    Content,
    Poster,
    Email,
    Url,
    Ip,
    Date,
    Hide,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Gid,
}
