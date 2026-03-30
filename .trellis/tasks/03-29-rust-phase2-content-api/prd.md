# Rust 内容管理 API - PRD

## 1. 概述

**任务名称**: Rust 内容管理 API
**所属阶段**: Phase 2 - Rust 接管
**优先级**: P0 (核心)
**预计工时**: 3 周
**前置依赖**: Rust 认证服务完成

## 2. 背景

Rust 核心服务需要实现内容管理 API，包括文章、分类、标签、评论的 CRUD 操作。

## 3. 目标

1. 实现文章 CRUD API
2. 实现分类管理 API
3. 实现标签管理 API
4. 实现评论管理 API
5. 编写集成测试

## 4. API 端点

```
# 文章管理
GET    /api/v2/posts          # 获取文章列表
GET    /api/v2/posts/:id      # 获取文章详情
POST   /api/v2/posts          # 创建文章
PUT    /api/v2/posts/:id      # 更新文章
DELETE /api/v2/posts/:id      # 删除文章

# 分类管理
GET    /api/v2/categories     # 获取分类列表
POST   /api/v2/categories     # 创建分类
PUT    /api/v2/categories/:id # 更新分类
DELETE /api/v2/categories/:id # 删除分类

# 标签管理
GET    /api/v2/tags           # 获取标签列表
POST   /api/v2/tags           # 创建标签
PUT    /api/v2/tags/:id       # 更新标签
DELETE /api/v2/tags/:id       # 删除标签

# 评论管理
GET    /api/v2/comments       # 获取评论列表
POST   /api/v2/comments       # 创建评论
PUT    /api/v2/comments/:id   # 更新评论
DELETE /api/v2/comments/:id   # 删除评论
```

## 5. 验收标准

- [ ] 所有 API 端点可正常调用
- [ ] 请求验证正确
- [ ] 响应格式统一
- [ ] 错误处理正确
- [ ] 集成测试通过

## 6. 任务清单

### 6.1 文章管理 API

- [ ] 实现 GET /posts (列表)
- [ ] 实现 GET /posts/:id (详情)
- [ ] 实现 POST /posts (创建)
- [ ] 实现 PUT /posts/:id (更新)
- [ ] 实现 DELETE /posts/:id (删除)
- [ ] 编写集成测试

### 6.2 分类管理 API

- [ ] 实现 GET /categories
- [ ] 实现 POST /categories
- [ ] 实现 PUT /categories/:id
- [ ] 实现 DELETE /categories/:id
- [ ] 编写集成测试

### 6.3 标签管理 API

- [ ] 实现 GET /tags
- [ ] 实现 POST /tags
- [ ] 实现 PUT /tags/:id
- [ ] 实现 DELETE /tags/:id
- [ ] 编写集成测试

### 6.4 评论管理 API

- [ ] 实现 GET /comments
- [ ] 实现 POST /comments
- [ ] 实现 PUT /comments/:id
- [ ] 实现 DELETE /comments/:id
- [ ] 编写集成测试

### 6.5 通用功能

- [ ] 实现请求验证
- [ ] 实现响应格式化
- [ ] 实现错误处理
- [ ] 实现分页
- [ ] 实现过滤和排序

## 7. 交付物

1. 内容管理 API 实现
2. 请求验证
3. 响应格式化
4. 集成测试

## 8. 响应格式

### 成功响应

```json
{
  "success": true,
  "data": {
    "id": 1,
    "title": "文章标题",
    "content": "文章内容"
  },
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
  "data": [
    {
      "id": 1,
      "title": "文章标题"
    }
  ],
  "meta": {
    "total": 100,
    "page": 1,
    "per_page": 20
  }
}
```

### 错误响应

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "请求数据验证失败",
    "details": []
  }
}
```

## 9. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| API 设计不一致 | 中 | 中 | 参考 RESTful 规范 |
| 性能问题 | 中 | 中 | 数据库查询优化 |
| 数据验证不严 | 高 | 中 | 完善验证规则 |

## 10. 参考文档

- [design-direction.md](../../spec/plog-rust-migration/design-direction.md)
- [migration-plan.md](../../spec/plog-rust-migration/migration-plan.md)
- [boundaries-and-protocols.md](../../spec/plog-rust-migration/boundaries-and-protocols.md)
