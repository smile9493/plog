# Rust 项目初始化 - PRD

## 1. 概述

**任务名称**: Rust 项目初始化
**所属阶段**: Phase 2 - Rust 接管
**优先级**: P0 (核心)
**预计工时**: 1 周
**前置依赖**: Phase 1 完成

## 2. 背景

Phase 1 完成边界收敛后，开始 Phase 2 的 Rust 服务开发。首先需要搭建 Rust 项目基础结构。

## 3. 目标

1. 创建 Cargo workspace
2. 配置项目结构
3. 设置 CI/CD
4. 配置开发环境

## 4. 技术栈

| 组件 | 选择 | 版本 |
|------|------|------|
| 语言 | Rust | 1.75+ |
| Web 框架 | Axum | 0.7+ |
| ORM | SeaORM | 1.0+ |
| 数据库驱动 | sqlx-mysql | - |
| 序列化 | serde | 1.0+ |
| 日志 | tracing | 0.1+ |
| 配置 | config | 0.14+ |

## 5. 项目结构

```
plog-rs/
├── Cargo.toml
├── crates/
│   ├── core/           # 核心库
│   ├── auth/           # 认证模块
│   ├── content/        # 内容模块
│   ├── plugin/         # 插件模块
│   ├── theme/          # 主题模块
│   └── api/            # API 服务
├── migrations/         # 数据库迁移
├── config/             # 配置文件
└── tests/              # 集成测试
```

## 6. 验收标准

- [ ] Cargo workspace 可编译
- [ ] 项目结构清晰
- [ ] CI/CD 配置完成
- [ ] 开发环境文档完整

## 7. 任务清单

### 7.1 Cargo Workspace

- [ ] 创建根 Cargo.toml
- [ ] 配置 workspace members
- [ ] 配置共享依赖
- [ ] 配置构建配置

### 7.2 Crate 结构

- [ ] 创建 core crate
- [ ] 创建 auth crate
- [ ] 创建 content crate
- [ ] 创建 plugin crate
- [ ] 创建 theme crate
- [ ] 创建 api crate

### 7.3 CI/CD 配置

- [ ] 配置 GitHub Actions
- [ ] 配置构建流程
- [ ] 配置测试流程
- [ ] 配置发布流程

### 7.4 开发环境

- [ ] 编写开发环境搭建文档
- [ ] 配置 IDE 支持
- [ ] 配置调试配置
- [ ] 配置代码格式化

## 8. 交付物

1. Cargo workspace 配置
2. 项目结构
3. CI/CD 配置
4. 开发环境文档

## 9. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| Rust 版本兼容性 | 中 | 低 | 固定 Rust 版本 |
| 依赖冲突 | 中 | 中 | 使用 Cargo.lock |
| CI/CD 配置复杂 | 低 | 中 | 参考成熟模板 |

## 10. 参考文档

- [design-direction.md](../../spec/plog-rust-migration/design-direction.md)
- [migration-plan.md](../../spec/plog-rust-migration/migration-plan.md)
