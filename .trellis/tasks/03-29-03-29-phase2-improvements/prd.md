# Phase2 改进 - API 统一响应与功能完善

## 1. 概述

**任务名称**: Phase2 改进 - API 统一响应与功能完善
**所属阶段**: Phase 2 - Rust 接管（改进）
**优先级**: P1 (重要)
**预计工时**: 1 周
**前置依赖**: Phase 2 核心任务完成

## 2. 背景

Phase 2 核心功能已完成，代码审查发现以下改进点需要处理。

## 3. 目标

1. 统一 API 响应格式
2. 实现筛选功能
3. 添加请求追踪
4. 编写单元测试
5. 生成 API 文档

## 4. 改进项清单

### 4.1 P1 - API 响应统一

**问题**: API 路由直接构建 JSON，未使用 `core::types::ApiResponse`

**修改文件**:
- `plog-rs/crates/api/src/routes/posts.rs`
- `plog-rs/crates/api/src/routes/categories.rs`
- `plog-rs/crates/api/src/routes/tags.rs`
- `plog-rs/crates/api/src/routes/comments.rs`

**修改内容**:
```rust
// Before
Json(serde_json::json!({
    "success": true,
    "data": posts
}))

// After
Json(ApiResponse::success(posts))
```

### 4.2 P1 - 筛选功能实现

**问题**: `ListParams` 中 `category_id`, `keyword`, `status`, `order` 字段未使用

**修改文件**:
- `plog-rs/crates/api/src/routes/posts.rs`
- `plog-rs/crates/content/src/repository/post.rs`

**实现内容**:
- 按分类筛选
- 关键词搜索
- 状态过滤
- 排序支持

### 4.3 P2 - 请求 ID 追踪

**问题**: 缺少请求 ID 追踪，不利于日志分析

**修改文件**:
- `plog-rs/crates/api/src/lib.rs` (添加中间件)
- `plog-rs/crates/core/src/types.rs` (已有 `request_id` 字段)

**实现内容**:
- 生成 UUID 作为请求 ID
- 传递到响应 header 和 body
- 注入到 tracing span

### 4.4 P2 - 单元测试

**问题**: 缺少单元测试覆盖

**修改文件**:
- `plog-rs/crates/auth/src/tests/` (新建)
- `plog-rs/crates/content/src/tests/` (新建)

**测试内容**:
- JWT 签发/验证
- 密码哈希/验证
- Repository CRUD

### 4.5 P3 - API 文档

**问题**: 缺少 API 文档

**修改文件**:
- `plog-rs/crates/api/src/lib.rs` (添加 OpenAPI 注解)
- `plog-rs/docs/api.md` (新建)

**实现内容**:
- OpenAPI 3.0 规范
- Swagger UI 集成

## 5. 验收标准

- [ ] 所有 API 使用统一 `ApiResponse` 格式
- [ ] 文章列表支持分类、关键词、状态筛选
- [ ] 每个响应包含 `request_id`
- [ ] 核心模块测试覆盖率 >= 60%
- [ ] API 文档可访问

## 6. 任务清单

- [ ] 统一 posts 路由响应格式
- [ ] 统一 categories 路由响应格式
- [ ] 统一 tags 路由响应格式
- [ ] 统一 comments 路由响应格式
- [ ] 实现文章筛选功能
- [ ] 添加请求 ID 中间件
- [ ] 编写 auth 模块测试
- [ ] 编写 content 模块测试
- [ ] 生成 API 文档

## 7. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| 响应格式变更影响前端 | 中 | 中 | 版本控制，渐进迁移 |
| 测试覆盖不足 | 低 | 中 | 聚焦核心路径 |

## 8. 参考文件

- `plog-rs/crates/core/src/types.rs` - 统一响应类型
- `.trellis/spec/plog-rust-migration/boundaries-and-protocols.md` - 响应格式规范
