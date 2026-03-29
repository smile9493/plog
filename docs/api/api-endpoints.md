# Plog CMS API 端点清单

## 概述

本文档列出 Plog CMS 系统的所有 API 端点，作为 PHP → Rust 迁移的接口契约基础。

**最后更新**: 2026-03-29
**版本**: v1.0.0

---

## API 端点总览

| 模块 | 端点数量 | 状态 |
|------|----------|------|
| 认证 | 3 | 已梳理 |
| 文章 | 7 | 已梳理 |
| 草稿 | 2 | 已梳理 |
| 分类 | 1 | 已梳理 |
| 标签 | - | 待梳理 |
| 评论 | 2 | 已梳理 |
| 笔记 | 2 | 已梳理 |
| 用户 | 2 | 已梳理 |
| 媒体 | 1 | 已梳理 |
| 点赞 | 2 | 已梳理 |

---

## 1. 认证模块 (Auth)

### 1.1 登录

- **端点**: `POST /api/auth/login`
- **认证**: 无需
- **请求体**:
```json
{
  "username": "admin",
  "password": "password123"
}
```
- **响应**:
```json
{
  "success": true,
  "data": {
    "token": "session_id_or_jwt",
    "user": {
      "uid": 1,
      "username": "admin",
      "nickname": "管理员",
      "role": "admin",
      "email": "admin@example.com"
    }
  }
}
```
- **错误响应**:
```json
{
  "success": false,
  "error": {
    "code": "INVALID_CREDENTIALS",
    "message": "用户名或密码错误"
  }
}
```

### 1.2 登出

- **端点**: `POST /api/auth/logout`
- **认证**: 需要
- **响应**:
```json
{
  "success": true,
  "message": "登出成功"
}
```

### 1.3 获取当前用户

- **端点**: `GET /api/auth/user`
- **认证**: 需要
- **响应**:
```json
{
  "success": true,
  "data": {
    "uid": 1,
    "username": "admin",
    "nickname": "管理员",
    "role": "admin",
    "email": "admin@example.com"
  }
}
```

---

## 2. 文章模块 (Posts)

### 2.1 文章列表

- **端点**: `GET /api/posts`
- **认证**: 可选
- **查询参数**:
  - `page` (int, 默认: 1): 页码
  - `count` (int, 默认: 20): 每页数量
  - `sort_id` (int, 可选): 分类 ID
  - `keyword` (string, 可选): 搜索关键词
  - `tag` (string, 可选): 标签名
  - `order` (string, 可选): 排序方式 (views, comnum, 默认: date)
- **响应**:
```json
{
  "success": true,
  "data": {
    "articles": [
      {
        "id": 1,
        "title": "文章标题",
        "cover": "https://example.com/cover.jpg",
        "url": "https://example.com/post/1",
        "description": "文章摘要",
        "date": "2026-03-29 10:00:00",
        "author_id": 1,
        "author_name": "管理员",
        "author_avatar": "https://example.com/avatar.jpg",
        "sort_id": 1,
        "sort_name": "技术",
        "views": 100,
        "comnum": 10,
        "like_count": 5,
        "top": "n",
        "sortop": "n",
        "tags": [
          {"name": "PHP", "url": "/tag/PHP"}
        ],
        "need_pwd": "n",
        "fields": {},
        "parent_id": 0
      }
    ],
    "page": 1,
    "total_pages": 10,
    "has_more": true
  }
}
```

### 2.2 文章详情

- **端点**: `GET /api/posts/{id}`
- **认证**: 可选
- **查询参数**:
  - `password` (string, 可选): 文章密码
- **响应**:
```json
{
  "success": true,
  "data": {
    "article": {
      "id": 1,
      "title": "文章标题",
      "date": "2026-03-29 10:00:00",
      "sort_id": 1,
      "sort_name": "技术",
      "type": "blog",
      "author_id": 1,
      "author_name": "管理员",
      "author_avatar": "https://example.com/avatar.jpg",
      "content": "<p>文章内容</p>",
      "content_raw": "文章内容",
      "excerpt": "文章摘要",
      "excerpt_raw": "文章摘要",
      "cover": "https://example.com/cover.jpg",
      "views": 100,
      "comnum": 10,
      "like_count": 5,
      "top": "n",
      "sortop": "n",
      "tags": [],
      "fields": {},
      "parent_id": 0
    }
  }
}
```

### 2.3 创建文章

- **端点**: `POST /api/posts`
- **认证**: 需要
- **请求体**:
```json
{
  "title": "文章标题",
  "content": "文章内容",
  "excerpt": "文章摘要",
  "author_uid": 1,
  "post_date": "2026-03-29 10:00:00",
  "sort_id": 1,
  "tags": "PHP,Rust",
  "cover": "https://example.com/cover.jpg",
  "draft": "n",
  "alias": "post-slug",
  "top": "n",
  "sortop": "n",
  "allow_remark": "y",
  "password": "",
  "link": "",
  "template": "",
  "field_keys": [],
  "field_values": [],
  "auto_cover": "n"
}
```
- **响应**:
```json
{
  "success": true,
  "data": {
    "article_id": 1
  }
}
```

### 2.4 更新文章

- **端点**: `POST /api/article/update`
- **认证**: 需要
- **请求体**: 同创建文章，额外包含 `id` 字段

### 2.5 草稿列表

- **端点**: `GET /api/draft/list`
- **认证**: 需要
- **查询参数**:
  - `count` (int, 默认: 20): 每页数量

### 2.6 草稿详情

- **端点**: `GET /api/draft/detail`
- **认证**: 需要
- **查询参数**:
  - `id` (int, 必需): 草稿 ID

---

## 3. 分类模块 (Categories)

### 3.1 分类列表

- **端点**: `GET /api/sort/list`
- **认证**: 无需
- **响应**:
```json
{
  "success": true,
  "data": {
    "sorts": {
      "1": {
        "sortname": "技术",
        "pid": 0,
        "children": [
          {
            "sid": 2,
            "sortname": "PHP"
          }
        ]
      }
    }
  }
}
```

---

## 4. 评论模块 (Comments)

### 4.1 评论列表

- **端点**: `GET /api/comment/list`
- **认证**: 无需
- **查询参数**:
  - `id` (int, 必需): 文章 ID
  - `page` (int, 默认: 1): 页码

### 4.2 评论列表 (简化版)

- **端点**: `GET /api/comment/list/simple`
- **认证**: 无需
- **查询参数**:
  - `id` (int, 必需): 文章 ID

---

## 5. 笔记模块 (Notes)

### 5.1 笔记列表

- **端点**: `GET /api/note/list`
- **认证**: 需要
- **查询参数**:
  - `page` (int, 默认: 1): 页码
  - `author_uid` (int, 可选): 作者 ID
  - `count` (int, 默认: 20): 每页数量

### 5.2 发布笔记

- **端点**: `POST /api/note/post`
- **认证**: 需要
- **请求体**:
```json
{
  "t": "笔记内容",
  "private": "n",
  "author_uid": 1
}
```

---

## 6. 用户模块 (Users)

### 6.1 获取用户信息

- **端点**: `GET /api/user/info`
- **认证**: 需要 (Cookie)

### 6.2 用户详情

- **端点**: `GET /api/user/detail`
- **认证**: 需要 (API Key)
- **查询参数**:
  - `id` (int, 必需): 用户 ID

---

## 7. 媒体模块 (Media)

### 7.1 上传文件

- **端点**: `POST /api/upload`
- **认证**: 需要 (API Key)
- **Content-Type**: `multipart/form-data`
- **表单字段**:
  - `file` (file, 必需): 文件
  - `sid` (int, 可选): 分类 ID
  - `author_uid` (int, 默认: 1): 作者 ID
- **响应**:
```json
{
  "success": true,
  "data": {
    "media_id": 1,
    "url": "https://example.com/uploads/file.jpg",
    "file_info": {}
  }
}
```

---

## 8. 点赞模块 (Likes)

### 8.1 取消点赞

- **端点**: `POST /api/unlike`
- **认证**: 需要 (Cookie)
- **请求体**:
```json
{
  "id": 1
}
```

### 8.2 点赞列表

- **端点**: `GET /api/like/list`
- **认证**: 无需
- **查询参数**:
  - `id` (int, 必需): 文章 ID

---

## 认证方式

系统支持两种认证方式:

### 1. Cookie 认证
- 登录后自动设置 Cookie
- 适用于 Web 前端

### 2. API Key 认证
- 通过 `api_key` 参数传递
- 或通过 `req_sign` 和 `req_time` 签名验证

---

## 错误码

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| VALIDATION_ERROR | 400 | 请求参数错误 |
| INVALID_CREDENTIALS | 401 | 用户名或密码错误 |
| UNAUTHORIZED | 401 | 未登录或认证失败 |
| USER_NOT_FOUND | 404 | 用户不存在 |
| METHOD_NOT_ALLOWED | 405 | 请求方法不允许 |
| DATABASE_ERROR | 500 | 数据库连接失败 |

---

## 待补充

- [ ] 标签管理 API (创建、更新、删除)
- [ ] 评论管理 API (创建、审核、删除)
- [ ] 用户管理 API (创建、更新、删除)
- [ ] 系统设置 API
- [ ] 插件管理 API
- [ ] 主题管理 API
