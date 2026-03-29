//! 数据库迁移

pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_user_table;
mod m20240101_000002_create_category_table;
mod m20240101_000003_create_post_table;
mod m20240101_000004_create_tag_table;
mod m20240101_000005_create_comment_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_user_table::Migration),
            Box::new(m20240101_000002_create_category_table::Migration),
            Box::new(m20240101_000003_create_post_table::Migration),
            Box::new(m20240101_000004_create_tag_table::Migration),
            Box::new(m20240101_000005_create_comment_table::Migration),
        ]
    }
}
