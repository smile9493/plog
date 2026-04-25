//! Plog CMS Domain Types
//! 
//! 类型安全的领域类型定义，替代魔法字符串和裸基本类型

use serde::{Deserialize, Serialize};

/// 用户角色枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Editor,
    User,
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin)
    }

    pub fn is_editor(&self) -> bool {
        matches!(self, Self::Admin | Self::Editor)
    }

    pub fn has_permission(&self, required: Role) -> bool {
        matches!((self, required),
            (_, Role::User) |
            (Role::Admin, _) |
            (Role::Editor, Role::Editor)
        )
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Editor => write!(f, "editor"),
            Self::User => write!(f, "user"),
        }
    }
}

impl From<Role> for String {
    fn from(role: Role) -> Self {
        role.to_string()
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Self::Admin),
            "editor" => Ok(Self::Editor),
            "user" => Ok(Self::User),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

/// 文章类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Blog,
    Page,
    Draft,
}

impl Default for PostType {
    fn default() -> Self {
        Self::Blog
    }
}

impl std::fmt::Display for PostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blog => write!(f, "blog"),
            Self::Page => write!(f, "page"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

impl From<PostType> for String {
    fn from(t: PostType) -> Self {
        t.to_string()
    }
}

/// 可见性（替代 "y"/"n" 字符串）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Visible(pub bool);

impl Default for Visible {
    fn default() -> Self {
        Self(true)
    }
}

impl From<Visible> for String {
    fn from(v: Visible) -> Self {
        if v.0 { "n" } else { "y" }.into()
    }
}

impl From<&str> for Visible {
    fn from(s: &str) -> Self {
        Self(s == "n")
    }
}

/// 置顶状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pinned(pub bool);

impl Default for Pinned {
    fn default() -> Self {
        Self(false)
    }
}

impl From<Pinned> for String {
    fn from(p: Pinned) -> Self {
        if p.0 { "y" } else { "n" }.into()
    }
}

/// 允许评论
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllowComment(pub bool);

impl Default for AllowComment {
    fn default() -> Self {
        Self(true)
    }
}

impl From<AllowComment> for String {
    fn from(a: AllowComment) -> Self {
        if a.0 { "y" } else { "n" }.into()
    }
}

/// 用户 ID（类型安全）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub i32);

impl From<i32> for UserId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<UserId> for i32 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

/// 文章 ID（类型安全）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PostId(pub i32);

impl From<i32> for PostId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<PostId> for i32 {
    fn from(id: PostId) -> Self {
        id.0
    }
}

/// 分类 ID（类型安全）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CategoryId(pub i32);

impl From<i32> for CategoryId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<CategoryId> for i32 {
    fn from(id: CategoryId) -> Self {
        id.0
    }
}

/// 标签 ID（类型安全）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TagId(pub i32);

impl From<i32> for TagId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<TagId> for i32 {
    fn from(id: TagId) -> Self {
        id.0
    }
}

/// 评论 ID（类型安全）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommentId(pub i32);

impl From<i32> for CommentId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<CommentId> for i32 {
    fn from(id: CommentId) -> Self {
        id.0
    }
}
