# Plog CMS API 响应格式规范

## 概述

本文档定义 Plog CMS API 的统一响应格式规范，确保所有 API 端点返回一致的响应结构。

**最后更新**: 2026-03-29
**版本**: v1.0.0

---

## 基本原则

1. **统一性**: 所有 API 使用相同的响应结构
2. **可预测性**: 客户端可以预期响应格式
3. **错误友好**: 错误信息清晰明确
4. **可扩展性**: 支持未来扩展

---

## 响应结构

### 成功响应

```json
{
  "success": true,
  "data": {
    // 实际数据
  },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

### 错误响应

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "错误描述",
    "details": []
  },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

---

## 字段说明

### success

- **类型**: boolean
- **说明**: 请求是否成功
- **必填**: 是

### data

- **类型**: object | array | null
- **说明**: 响应数据
- **必填**: 成功时必填

### error

- **类型**: object
- **说明**: 错误信息
- **必填**: 失败时必填

### meta

- **类型**: object
- **说明**: 元数据
- **必填**: 否

---

## 分页响应

### 结构

```json
{
  "success": true,
  "data": {
    "items": [],
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

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| page | int | 当前页码 |
| per_page | int | 每页数量 |
| total | int | 总记录数 |
| total_pages | int | 总页数 |
| has_more | boolean | 是否有更多 |

---

## 错误码规范

### 格式

```
{MODULE}_{ERROR_TYPE}
```

### 认证错误 (AUTH_)

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| AUTH_REQUIRED | 401 | 需要认证 |
| AUTH_FAILED | 401 | 认证失败 |
| AUTH_EXPIRED | 401 | 认证已过期 |
| AUTH_INVALID_TOKEN | 401 | 无效的 Token |

### 权限错误 (PERMISSION_)

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| PERMISSION_DENIED | 403 | 权限不足 |
| PERMISSION_INSUFFICIENT | 403 | 权限不足 |

### 验证错误 (VALIDATION_)

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| VALIDATION_ERROR | 422 | 验证失败 |
| VALIDATION_REQUIRED | 422 | 必填字段缺失 |
| VALIDATION_INVALID | 422 | 字段值无效 |

### 资源错误 (RESOURCE_)

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| RESOURCE_NOT_FOUND | 404 | 资源不存在 |
| RESOURCE_ALREADY_EXISTS | 409 | 资源已存在 |
| RESOURCE_CONFLICT | 409 | 资源冲突 |

### 服务器错误 (SERVER_)

| 错误码 | HTTP 状态码 | 说明 |
|--------|-------------|------|
| SERVER_ERROR | 500 | 服务器内部错误 |
| SERVER_UNAVAILABLE | 503 | 服务不可用 |
| SERVER_TIMEOUT | 504 | 请求超时 |

---

## HTTP 状态码

### 成功

| 状态码 | 说明 |
|--------|------|
| 200 | 请求成功 |
| 201 | 创建成功 |
| 204 | 无内容 |

### 客户端错误

| 状态码 | 说明 |
|--------|------|
| 400 | 请求错误 |
| 401 | 未授权 |
| 403 | 禁止访问 |
| 404 | 未找到 |
| 405 | 方法不允许 |
| 409 | 冲突 |
| 422 | 验证失败 |
| 429 | 请求过多 |

### 服务器错误

| 状态码 | 说明 |
|--------|------|
| 500 | 服务器错误 |
| 502 | 网关错误 |
| 503 | 服务不可用 |
| 504 | 网关超时 |

---

## 响应示例

### 文章列表

```json
{
  "success": true,
  "data": {
    "articles": [
      {
        "id": 1,
        "title": "文章标题",
        "excerpt": "文章摘要",
        "cover": "https://example.com/cover.jpg",
        "author": {
          "id": 1,
          "name": "管理员",
          "avatar": "https://example.com/avatar.jpg"
        },
        "category": {
          "id": 1,
          "name": "技术"
        },
        "tags": [
          {"id": 1, "name": "PHP"},
          {"id": 2, "name": "Rust"}
        ],
        "stats": {
          "views": 100,
          "comments": 10,
          "likes": 5
        },
        "date": "2026-03-29T10:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 100,
      "total_pages": 5,
      "has_more": true
    }
  },
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

### 验证错误

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "请求数据验证失败",
    "details": [
      {
        "field": "title",
        "message": "标题不能为空"
      },
      {
        "field": "email",
        "message": "邮箱格式不正确"
      }
    ]
  },
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

### 未授权

```json
{
  "success": false,
  "error": {
    "code": "AUTH_REQUIRED",
    "message": "请先登录"
  },
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

### 资源不存在

```json
{
  "success": false,
  "error": {
    "code": "RESOURCE_NOT_FOUND",
    "message": "文章不存在"
  },
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2026-03-29T10:00:00Z"
  }
}
```

---

## 响应头

### 标准响应头

```http
Content-Type: application/json; charset=utf-8
X-Request-ID: 550e8400-e29b-41d4-a716-446655440000
X-API-Version: 1.0.0
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1648627200
```

### CORS 响应头

```http
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization
Access-Control-Max-Age: 86400
```

---

## 最佳实践

### 1. 始终返回一致的结构

即使是空数据，也要保持结构一致：

```json
{
  "success": true,
  "data": {
    "articles": []
  }
}
```

### 2. 使用有意义的错误码

避免使用通用错误码，使用具体的错误码：

```json
// 不好
{"code": "ERROR"}

// 好
{"code": "VALIDATION_REQUIRED_FIELD_MISSING"}
```

### 3. 提供有用的错误消息

```json
// 不好
{"message": "验证失败"}

// 好
{"message": "标题不能为空，请提供有效的标题"}
```

### 4. 包含请求 ID

便于问题追踪：

```json
{
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

### 5. 使用 ISO 8601 时间格式

```json
{
  "date": "2026-03-29T10:00:00Z"
}
```
