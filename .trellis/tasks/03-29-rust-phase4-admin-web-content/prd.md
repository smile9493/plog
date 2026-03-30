# admin-web 内容管理 - PRD

## 1. 概述

**任务名称**: admin-web 内容管理
**所属阶段**: Phase 4 - 逐步退休 /admin
**优先级**: P0 (核心)
**预计工时**: 3 周
**前置依赖**: admin-web 用户管理完成

## 2. 背景

admin-web 内容管理是最重要的功能模块，包括文章、分类、标签、评论的管理。完成后可下线 /admin 中的内容管理页面。

## 3. 目标

1. 实现文章管理页面
2. 实现分类管理页面
3. 实现标签管理页面
4. 实现评论管理页面
5. 实现媒体管理页面

## 4. 页面清单

| 页面 | 路由 | 功能 |
|------|------|------|
| 文章列表 | /admin-web/posts | 文章列表、搜索、筛选 |
| 文章编辑 | /admin-web/posts/:id | 文章编辑、发布 |
| 文章创建 | /admin-web/posts/create | 创建新文章 |
| 分类管理 | /admin-web/categories | 分类列表、创建、编辑 |
| 标签管理 | /admin-web/tags | 标签列表、创建、编辑 |
| 评论管理 | /admin-web/comments | 评论列表、审核、回复 |
| 媒体管理 | /admin-web/media | 媒体上传、管理 |

## 5. 验收标准

- [ ] 所有页面可正常访问
- [ ] CRUD 操作正常
- [ ] 富文本编辑器正常
- [ ] 媒体上传正常
- [ ] 用户体验良好

## 6. 任务清单

### 6.1 文章管理

- [ ] 实现文章列表页面
- [ ] 实现文章编辑页面
- [ ] 实现文章创建页面
- [ ] 集成富文本编辑器
- [ ] 实现文章发布流程

### 6.2 分类管理

- [ ] 实现分类列表页面
- [ ] 实现分类创建/编辑
- [ ] 实现分类层级管理
- [ ] 实现分类排序

### 6.3 标签管理

- [ ] 实现标签列表页面
- [ ] 实现标签创建/编辑
- [ ] 实现标签关联

### 6.4 评论管理

- [ ] 实现评论列表页面
- [ ] 实现评论审核
- [ ] 实现评论回复
- [ ] 实现评论删除

### 6.5 媒体管理

- [ ] 实现媒体列表页面
- [ ] 实现媒体上传
- [ ] 实现媒体管理
- [ ] 实现媒体选择器

## 7. 交付物

1. 文章管理页面
2. 分类管理页面
3. 标签管理页面
4. 评论管理页面
5. 媒体管理页面
6. 富文本编辑器集成
7. 测试

## 8. API 集成

```
# 文章
GET    /api/v2/posts          # 获取文章列表
GET    /api/v2/posts/:id      # 获取文章详情
POST   /api/v2/posts          # 创建文章
PUT    /api/v2/posts/:id      # 更新文章
DELETE /api/v2/posts/:id      # 删除文章

# 分类
GET    /api/v2/categories     # 获取分类列表
POST   /api/v2/categories     # 创建分类
PUT    /api/v2/categories/:id # 更新分类
DELETE /api/v2/categories/:id # 删除分类

# 标签
GET    /api/v2/tags           # 获取标签列表
POST   /api/v2/tags           # 创建标签
PUT    /api/v2/tags/:id       # 更新标签
DELETE /api/v2/tags/:id       # 删除标签

# 评论
GET    /api/v2/comments       # 获取评论列表
POST   /api/v2/comments       # 创建评论
PUT    /api/v2/comments/:id   # 更新评论
DELETE /api/v2/comments/:id   # 删除评论

# 媒体
GET    /api/v2/media          # 获取媒体列表
POST   /api/v2/media          # 上传媒体
DELETE /api/v2/media/:id      # 删除媒体
```

## 9. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| 富文本编辑器兼容性 | 中 | 中 | 选择成熟方案 |
| 媒体上传性能 | 中 | 中 | 分片上传 |
| 数据量大导致页面卡顿 | 中 | 中 | 虚拟滚动 |

## 10. 参考文档

- [design-direction.md](../../spec/plog-rust-migration/design-direction.md)
- [migration-plan.md](../../spec/plog-rust-migration/migration-plan.md)
