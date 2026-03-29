//! 请求格式定义

use serde::{Deserialize, Serialize};

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub role: String,
    pub avatar: Option<String>,
}

/// 创建文章请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub category_id: Option<i32>,
    pub cover: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
}

/// 更新文章请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub category_id: Option<i32>,
    pub cover: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
}

/// 创建分类请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
}

/// 更新分类请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
}

/// 创建标签请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: Option<String>,
}

/// 更新标签请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
}

/// 创建评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub post_id: i32,
    pub content: String,
    pub author: String,
    pub email: Option<String>,
    pub url: Option<String>,
    pub parent_id: Option<i32>,
}

/// 创建用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: String,
    pub role: Option<String>,
}

/// 更新用户请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub avatar: Option<String>,
}

/// 重置密码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
}
