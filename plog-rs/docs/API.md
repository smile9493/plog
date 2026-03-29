# Plog CMS API v2 文档

## 概述

Plog CMS API v2 是基于 Rust (Axum) 构建的 RESTful API，提供文章、分类、标签、评论的管理功能。

**Base URL**: `https://admin.example.com/api/v2`

---

## 认证

所有写操作需要 JWT Token 认证。

### 获取 Token

```http
POST /api/v2/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password"
}
```

**响应**:
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600,
    "user": {
      "id": 1,
      "username": "admin",
      "nickname": "Administrator",
      "role": "admin",
      "email": "admin@example.com"
    }
  }
}
```

### 使用 Token

在请求头中添加：
```
Authorization: Bearer <token>
```

### 获取当前用户

```http
GET /api/v2/auth/me
Authorization: Bearer <token>
```

---

## 通用响应格式

### 成功响应
```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

### 列表响应
```json
{
  "success": true,
  "data": {
    "items": [ ... ],
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 100,
      "total_pages": 5,
      "has_more": true
    }
  }
}
```

### 错误响应
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error description"
  }
}
```

---

## 文章 API

### 获取文章列表

```http
GET /api/v2/posts
```

**查询参数**:

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| page | u64 | 否 | 页码，默认 1 |
| per_page | u64 | 否 | 每页数量，默认 20，最大 100 |
| keyword | string | 否 | 关键词搜索 (标题和内容) |
| category_id | i32 | 否 | 分类 ID 筛选 |
| status | string | 否 | 状态: `published` / `draft` |
| order | string | 否 | 排序: `views` / `comments` / `likes` / `title` |

**示例**:
```http
GET /api/v2/posts?page=1&per_page=10&keyword=rust&status=published&order=views
```

### 获取文章详情

```http
GET /api/v2/posts/:id
```

**路径参数**:

| 参数 | 类型 | 说明 |
|------|------|------|
| id | i32 | 文章 ID |

### 创建文章

```http
POST /api/v2/posts
Authorization: Bearer <token>
Content-Type: application/json
```

**请求体**:

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| title | string | 是 | 文章标题 |
| content | string | 是 | 文章内容 |
| excerpt | string | 否 | 文章摘要 |
| sortid | i32 | 否 | 分类 ID |
| cover | string | 否 | 封面图片 URL |
| alias | string | 否 | 文章别名 (URL slug) |
| hide | string | 否 | 发布状态: `y`=草稿, `n`=发布 |
| top | string | 否 | 是否置顶: `y`/`n` |
| allow_remark | string | 否 | 允许评论: `y`/`n` |
| password | string | 否 | 文章密码 |
| type | string | 否 | 文章类型，默认 `blog` |

**示例**:
```json
{
  "title": "Rust 入门教程",
  "content": "# Rust 简介\n\nRust 是一门系统编程语言...",
  "excerpt": "Rust 基础教程",
  "sortid": 1,
  "hide": "n"
}
```

### 更新文章

```http
PUT /api/v2/posts/:id
Authorization: Bearer <token>
Content-Type: application/json
```

**请求体**: 同创建文章，所有字段可选

### 删除文章

```http
DELETE /api/v2/posts/:id
Authorization: Bearer <token>
```

---

## 分类 API

### 获取分类列表

```http
GET /api/v2/categories
```

### 获取分类详情

```http
GET /api/v2/categories/:id
```

### 创建分类

```http
POST /api/v2/categories
Authorization: Bearer <token>
Content-Type: application/json
```

**请求体**:

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| sortname | string | 是 | 分类名称 |
| pid | i32 | 否 | 父分类 ID (0=顶级) |
| sortorder | i32 | 否 | 排序权重 |
| description | string | 否 | 分类描述 |
| alias | string | 否 | 分类别名 |

### 更新分类

```http
PUT /api/v2/categories/:id
Authorization: Bearer <token>
Content-Type: application/json
```

### 删除分类

```http
DELETE /api/v2/categories/:id
Authorization: Bearer <token>
```

---

## 标签 API

### 获取标签列表

```http
GET /api/v2/tags
```

**查询参数**:

| 参数 | 类型 | 说明 |
|------|------|------|
| limit | u64 | 限制返回数量 |
| popular | bool | 是否只返回热门标签 |

**示例**:
```http
GET /api/v2/tags?popular=true&limit=10
```

### 获取标签详情

```http
GET /api/v2/tags/:id
```

### 创建标签

```http
POST /api/v2/tags
Authorization: Bearer <token>
Content-Type: application/json
```

**请求体**:

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| tagname | string | 是 | 标签名称 |

### 更新标签

```http
PUT /api/v2/tags/:id
Authorization: Bearer <token>
Content-Type: application/json
```

### 删除标签

```http
DELETE /api/v2/tags/:id
Authorization: Bearer <token>
```

---

## 评论 API

### 获取评论列表

```http
GET /api/v2/comments
```

**查询参数**:

| 参数 | 类型 | 说明 |
|------|------|------|
| page | u64 | 页码 |
| per_page | u64 | 每页数量 |
| post_id | i32 | 筛选指定文章的评论 |
| status | string | 状态筛选 |

### 获取评论详情

```http
GET /api/v2/comments/:id
```

### 创建评论

```http
POST /api/v2/comments
Authorization: Bearer <token>
Content-Type: application/json
```

**请求体**:

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| gid | i32 | 是 | 文章 ID |
| content | string | 是 | 评论内容 |
| poster | string | 是 | 评论者名称 |
| pid | i32 | 否 | 父评论 ID |
| email | string | 否 | 评论者邮箱 |
| url | string | 否 | 评论者网站 |

### 更新评论

```http
PUT /api/v2/comments/:id
Authorization: Bearer <token>
Content-Type: application/json
```

### 删除评论

```http
DELETE /api/v2/comments/:id
Authorization: Bearer <token>
```

### 审核评论

```http
POST /api/v2/comments/:id/approve
Authorization: Bearer <token>
```

---

## 错误码

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| VALIDATION_ERROR | 422 | 请求数据验证失败 |
| NOT_FOUND | 404 | 资源不存在 |
| AUTH_ERROR | 401 | 认证失败 |
| PERMISSION_DENIED | 403 | 权限不足 |
| DATABASE_ERROR | 500 | 数据库错误 |
| INTERNAL_ERROR | 500 | 服务器内部错误 |

---

## 请求追踪

每个请求会生成唯一的 `request_id`，可通过以下方式获取:

1. **响应头**: `X-Request-ID`
2. **响应体**: `meta.request_id`

用于日志查询和问题排查。

---

## 版本信息

| 版本 | 状态 | 说明 |
|------|------|------|
| v1 | 维护中 | 旧版 PHP API |
| v2 | 开发中 | 新版 Rust API |
