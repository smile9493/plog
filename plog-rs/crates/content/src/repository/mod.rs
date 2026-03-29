//! Repository 层

pub mod user;
pub mod post;
pub mod category;
pub mod tag;
pub mod comment;

pub use user::UserRepository;
pub use post::PostRepository;
pub use category::CategoryRepository;
pub use tag::TagRepository;
pub use comment::CommentRepository;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// Repository 上下文
#[derive(Clone)]
pub struct RepositoryContext {
    pub db: Arc<DatabaseConnection>,
}

impl RepositoryContext {
    /// 创建新的 Repository 上下文
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db: Arc::new(db),
        }
    }

    /// 获取用户 Repository
    pub fn users(&self) -> UserRepository {
        UserRepository::new(self.db.clone())
    }

    /// 获取文章 Repository
    pub fn posts(&self) -> PostRepository {
        PostRepository::new(self.db.clone())
    }

    /// 获取分类 Repository
    pub fn categories(&self) -> CategoryRepository {
        CategoryRepository::new(self.db.clone())
    }

    /// 获取标签 Repository
    pub fn tags(&self) -> TagRepository {
        TagRepository::new(self.db.clone())
    }

    /// 获取评论 Repository
    pub fn comments(&self) -> CommentRepository {
        CommentRepository::new(self.db.clone())
    }
}
