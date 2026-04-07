//! Plog CMS Content Module
//!
//! 内容管理模块

pub mod entities;
pub mod repository;

pub use entities::{
    user,
    post,
    category,
    tag,
    comment,
};
pub use repository::{
    RepositoryContext,
    UserRepository,
    PostRepository,
    CategoryRepository,
    TagRepository,
    CommentRepository,
};

#[cfg(test)]
mod tests {
    use crate::post::Model as PostModel;
    use crate::tag::Model as TagModel;
    use crate::category::Model as CategoryModel;
    use crate::comment::Model as CommentModel;

    #[test]
    fn test_post_model_creation() {
        let post = PostModel {
            gid: 1,
            title: "Test Post".to_string(),
            content: "Content here".to_string(),
            excerpt: Some("Excerpt".to_string()),
            author: 1,
            sortid: 0,
            date: 1700000000,
            hide: "n".to_string(),
            r#type: "blog".to_string(),
            views: 0,
            comnum: 0,
            like_count: 0,
            top: "n".to_string(),
            sortop: "n".to_string(),
            allow_remark: "y".to_string(),
            password: None,
            cover: None,
            alias: None,
        };
        assert_eq!(post.gid, 1);
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.hide, "n");
        assert_eq!(post.views, 0);
    }

    #[test]
    fn test_post_serialization() {
        let post = PostModel {
            gid: 1,
            title: "Test".to_string(),
            content: "Content".to_string(),
            excerpt: None,
            author: 1,
            sortid: 0,
            date: 1700000000,
            hide: "n".to_string(),
            r#type: "blog".to_string(),
            views: 100,
            comnum: 5,
            like_count: 10,
            top: "n".to_string(),
            sortop: "n".to_string(),
            allow_remark: "y".to_string(),
            password: None,
            cover: Some("http://example.com/cover.jpg".to_string()),
            alias: Some("test-post".to_string()),
        };
        let json = serde_json::to_string(&post).unwrap();
        assert!(json.contains("\"title\":\"Test\""));
        assert!(json.contains("\"views\":100"));
    }

    #[test]
    fn test_tag_model() {
        let tag = TagModel {
            tid: 1,
            tagname: "rust".to_string(),
            usenum: 5,
        };
        assert_eq!(tag.tid, 1);
        assert_eq!(tag.tagname, "rust");
        assert_eq!(tag.usenum, 5);
    }

    #[test]
    fn test_category_model() {
        let cat = CategoryModel {
            sid: 1,
            sortname: "Tech".to_string(),
            pid: 0,
            sortorder: 0,
            description: Some("Tech posts".to_string()),
            alias: Some("tech".to_string()),
        };
        assert_eq!(cat.sid, 1);
        assert_eq!(cat.sortname, "Tech");
    }

    #[test]
    fn test_comment_model() {
        let comment = CommentModel {
            cid: 1,
            gid: 10,
            pid: 0,
            content: "Nice post!".to_string(),
            poster: "John".to_string(),
            email: "john@example.com".to_string(),
            url: "".to_string(),
            ip: "127.0.0.1".to_string(),
            date: 1700000000,
            hide: "y".to_string(),
        };
        assert_eq!(comment.cid, 1);
        assert_eq!(comment.gid, 10);
        assert_eq!(comment.hide, "y");
    }
}
