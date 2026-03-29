//! Content 模块单元测试

use plog_content::entities::{category, comment, post, tag, user};

/// 测试实体模型结构
#[test]
fn test_post_model_structure() {
    let post = post::Model {
        gid: 1,
        title: "Test Post".to_string(),
        content: "Test Content".to_string(),
        excerpt: Some("Test Excerpt".to_string()),
        author: 1,
        sortid: 1,
        date: 1640000000,
        hide: "n".to_string(),
        r#type: "blog".to_string(),
        views: 0,
        comnum: 0,
        like_count: 0,
        top: "n".to_string(),
        sortop: "n".to_string(),
        allow_remark: "y".to_string(),
        password: None,
        cover: Some("cover.jpg".to_string()),
        alias: Some("test-post".to_string()),
    };

    assert_eq!(post.gid, 1);
    assert_eq!(post.title, "Test Post");
    assert_eq!(post.content, "Test Content");
    assert_eq!(post.hide, "n");
    assert_eq!(post.r#type, "blog");
}

/// 测试用户模型结构
#[test]
fn test_user_model_structure() {
    let user = user::Model {
        uid: 1,
        username: "testuser".to_string(),
        password: "hashed_password".to_string(),
        nickname: "Test User".to_string(),
        role: "admin".to_string(),
        email: Some("test@example.com".to_string()),
        photo: Some("avatar.jpg".to_string()),
        description: Some("Test description".to_string()),
        create_time: 1640000000,
    };

    assert_eq!(user.uid, 1);
    assert_eq!(user.username, "testuser");
    assert_eq!(user.role, "admin");
    assert_eq!(user.email, Some("test@example.com".to_string()));
}

/// 测试分类模型结构
#[test]
fn test_category_model_structure() {
    let category = category::Model {
        sid: 1,
        sortname: "Technology".to_string(),
        pid: 0,
        sortorder: 1,
        description: Some("Technology articles".to_string()),
        alias: Some("tech".to_string()),
    };

    assert_eq!(category.sid, 1);
    assert_eq!(category.sortname, "Technology");
    assert_eq!(category.pid, 0);
    assert_eq!(category.sortorder, 1);
}

/// 测试标签模型结构
#[test]
fn test_tag_model_structure() {
    let tag = tag::Model {
        tid: 1,
        tagname: "Rust".to_string(),
        usenum: 10,
    };

    assert_eq!(tag.tid, 1);
    assert_eq!(tag.tagname, "Rust");
    assert_eq!(tag.usenum, 10);
}

/// 测试评论模型结构
#[test]
fn test_comment_model_structure() {
    let comment = comment::Model {
        cid: 1,
        gid: 1, // 文章 ID
        pid: 0, // 父评论 ID
        content: "Great article!".to_string(),
        poster: "Anonymous".to_string(),
        email: "anon@example.com".to_string(),
        url: "https://example.com".to_string(),
        ip: "127.0.0.1".to_string(),
        date: 1640000000,
        hide: "n".to_string(),
    };

    assert_eq!(comment.cid, 1);
    assert_eq!(comment.gid, 1);
    assert_eq!(comment.content, "Great article!");
    assert_eq!(comment.poster, "Anonymous");
    assert_eq!(comment.hide, "n");
}

/// 测试文章模型克隆
#[test]
fn test_post_model_clone() {
    let post = post::Model {
        gid: 1,
        title: "Test Post".to_string(),
        content: "Test Content".to_string(),
        excerpt: None,
        author: 1,
        sortid: 1,
        date: 1640000000,
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

    let cloned = post.clone();
    assert_eq!(post, cloned);
}

/// 测试文章模型相等性
#[test]
fn test_post_model_equality() {
    let post1 = post::Model {
        gid: 1,
        title: "Test".to_string(),
        content: "Content".to_string(),
        excerpt: None,
        author: 1,
        sortid: 1,
        date: 1640000000,
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

    let post2 = post::Model {
        gid: 1,
        title: "Test".to_string(),
        content: "Content".to_string(),
        excerpt: None,
        author: 1,
        sortid: 1,
        date: 1640000000,
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

    assert_eq!(post1, post2);
}

/// 测试文章模型调试输出
#[test]
fn test_post_model_debug() {
    let post = post::Model {
        gid: 1,
        title: "Test".to_string(),
        content: "Content".to_string(),
        excerpt: None,
        author: 1,
        sortid: 1,
        date: 1640000000,
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

    let debug_str = format!("{:?}", post);
    assert!(debug_str.contains("gid: 1"));
    assert!(debug_str.contains("title: \"Test\""));
}

/// 测试用户模型部分相等
#[test]
fn test_user_model_partial_equality() {
    let user1 = user::Model {
        uid: 1,
        username: "user1".to_string(),
        password: "pass1".to_string(),
        nickname: "User One".to_string(),
        role: "admin".to_string(),
        email: None,
        photo: None,
        description: None,
        create_time: 1640000000,
    };

    let user2 = user::Model {
        uid: 1,
        username: "user1".to_string(),
        password: "pass1".to_string(),
        nickname: "User One".to_string(),
        role: "admin".to_string(),
        email: None,
        photo: None,
        description: None,
        create_time: 1640000000,
    };

    assert_eq!(user1, user2);
}
