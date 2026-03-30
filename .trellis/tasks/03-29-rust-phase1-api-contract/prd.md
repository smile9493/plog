# API 接口梳理与契约固定 - PRD

## 1. 概述

**任务名称**: API 接口梳理与契约固定
**所属阶段**: Phase 1 - 边界收敛
**优先级**: P0 (核心)
**预计工时**: 2 周
**前置依赖**: M1 阶段完成 (PHP Monorepo 重构)

## 2. 背景

在进行 PHP → Rust 迁移之前，需要先固定现有系统的 API 接口契约。这是 Phase 1 边界收敛的核心任务，为后续 Rust 接管 API 服务做准备。

## 3. 目标

1. 梳理现有 admin-api 的所有 API 端点
2. 编写 OpenAPI/Swagger 规范文档
3. 定义 API 版本管理策略
4. 建立 API 测试套件

## 4. 范围

### 包含

- 现有 API 端点清单
- OpenAPI 规范文档
- 版本管理策略
- API 测试套件
- 响应格式规范
- 错误码规范

### 不包含

- Rust 服务实现
- API 性能优化
- 新增 API 功能

## 5. 验收标准

- [ ] 所有 API 端点有明确的接口契约
- [ ] OpenAPI 规范文档完整
- [ ] 版本管理策略文档完成
- [ ] API 测试套件可运行
- [ ] 现有功能不受影响

## 6. 任务清单

### 6.1 API 端点梳理

- [ ] 梳理认证相关 API
- [ ] 梳理内容管理 API
- [ ] 梳理分类管理 API
- [ ] 梳理标签管理 API
- [ ] 梳理评论管理 API
- [ ] 梳理媒体管理 API
- [ ] 梳理用户管理 API
- [ ] 梳理系统设置 API

### 6.2 OpenAPI 规范编写

- [ ] 定义 Info 对象
- [ ] 定义 Server 对象
- [ ] 定义 Path 对象
- [ ] 定义 Schema 对象
- [ ] 定义 Response 对象
- [ ] 定义 SecurityScheme 对象

### 6.3 版本管理策略

- [ ] 定义版本号规范
- [ ] 定义版本生命周期
- [ ] 定义版本迁移策略
- [ ] 编写版本管理文档

### 6.4 API 测试套件

- [ ] 编写认证 API 测试
- [ ] 编写内容 API 测试
- [ ] 编写分类 API 测试
- [ ] 编写标签 API 测试
- [ ] 编写评论 API 测试

## 7. 交付物

1. API 端点清单文档
2. OpenAPI 规范文档 (YAML/JSON)
3. 版本管理策略文档
4. API 测试套件
5. 响应格式规范文档
6. 错误码规范文档

## 8. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| 现有 API 缺乏文档 | 中 | 高 | 通过代码分析补充 |
| API 设计不一致 | 中 | 中 | 统一规范后修正 |
| 测试覆盖困难 | 低 | 中 | 优先测试核心接口 |

## 9. 参考文档

- [design-direction.md](../../spec/plog-rust-migration/design-direction.md)
- [migration-plan.md](../../spec/plog-rust-migration/migration-plan.md)
- [boundaries-and-protocols.md](../../spec/plog-rust-migration/boundaries-and-protocols.md)
