# Plog CMS PHP → Rust 长期重构规范

## 概述

本目录包含 Plog CMS 从 PHP 单体架构向 Rust 核心 + PHP 兼容层架构迁移的长期重构设计规范。

## 核心理念

**不是把 PHP 改成 Rust，而是把系统核心逐步"Rust 化"，PHP 最后只剩兼容层。**

## 文档目录

| 文档 | 说明 | 状态 |
|------|------|------|
| [design-direction.md](design-direction.md) | 重构设计方向文档 | 草案 |
| [migration-plan.md](migration-plan.md) | 分阶段迁移实施计划 | 草案 |
| [boundaries-and-protocols.md](boundaries-and-protocols.md) | 技术边界与交互协议 | 草案 |

## 快速开始

### 1. 理解重构方向

阅读 [design-direction.md](design-direction.md) 了解:
- 重构哲学
- 职责划分 (Rust vs PHP)
- 分阶段路线图
- 最终目标架构

### 2. 了解迁移计划

阅读 [migration-plan.md](migration-plan.md) 了解:
- 各阶段详细任务
- 时间线和里程碑
- 资源需求
- 风险评估

### 3. 理解技术边界

阅读 [boundaries-and-protocols.md](boundaries-and-protocols.md) 了解:
- 系统边界定义
- 交互协议规范
- 数据所有权规则
- 安全和监控边界

## 与 M1 阶段的关系

本重构方向建立在 M1 阶段 (PHP Monorepo 重构) 的基础上:

```
M1 阶段 (PHP Monorepo 重构) ──► Phase 1 (边界收敛)
                                      │
                                      ▼
                               Phase 2 (Rust 接管)
                                      │
                                      ▼
                               Phase 3 (掏空 PHP)
                                      │
                                      ▼
                               Phase 4 (退休 /admin)
```

M1 阶段的工作为 Rust 迁移做准备:
- Monorepo 结构 - 为多语言项目管理打基础
- 接口抽象 - 定义清晰的领域边界
- API 规范 - 固定 API 契约，便于 Rust 接管
- Manifest 驱动 - 为插件/主题元数据标准化做准备

## 迁移时间线

| 阶段 | 时间 | 目标 |
|------|------|------|
| Phase 1 | 2026 Q1-Q2 | 边界收敛，接口契约固定 |
| Phase 2 | 2026 Q3-Q4 | Rust 核心服务上线 |
| Phase 3 | 2027 Q1-Q2 | PHP 代码量减少 60% |
| Phase 4 | 2027 Q3-Q4 | /admin 页面退休，迁移完成 |

## 关键决策

### 为什么选择 HTTP/JSON 而非 FFI?

1. 符合现有 Web/CMS 架构模式
2. 调试和监控简单
3. 团队学习成本低
4. 部署独立，互不影响
5. 便于水平扩展

### 为什么选择 Axum 而非 Actix-web?

1. 基于 tokio/tower 生态
2. 更好的类型安全
3. 中间件系统灵活
4. 社区活跃度高

### 为什么选择 SeaORM 而非 Diesel?

1. 异步支持好
2. 实体关系定义清晰
3. 迁移工具完善
4. 与 Axum 集成良好

## 变更历史

| 版本 | 日期 | 变更内容 | 作者 |
|-----|------|---------|------|
| v1.0.0 | 2026-03-29 | 初始版本创建 | AI Agent |
