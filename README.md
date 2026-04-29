# Plog CMS

> 基于 Rust 的现代博客系统，采用微内核架构，支持 Docker 一键部署

[![Rust](https://img.shields.io/badge/Rust-1.88+-orange.svg)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## 技术栈

| 层级 | 技术 |
|------|------|
| 后端 | Rust、Axum、SeaORM、MySQL |
| 前端管理 | Vue 3、TypeScript、Element Plus |
| 前台网站 | HTML/CSS/JS、多主题支持 |
| 部署 | Docker、Docker Compose、Nginx |

## 架构

```
plog/
├── plog-rs/                    # Rust 微内核工作区
│   ├── packages/               # 基础包
│   │   ├── plog-core/          # 核心错误与工具
│   │   └── plog-shared/        # 共享类型与 trait
│   ├── modules/                # 功能模块
│   │   ├── content/            # 内容管理
│   │   ├── auth/               # 认证授权
│   │   ├── settings/           # 系统设置
│   │   ├── media/              # 媒体管理
│   │   └── audit/              # 审计日志
│   ├── extensions/             # 扩展模块
│   │   ├── plugin/             # 插件系统
│   │   └── theme/              # 主题系统
│   ├── plog-api/               # API 服务入口
│   ├── crates/                 # 独立测试模块
│   ├── config/                 # 配置文件
│   ├── migrations/             # 数据库迁移
│   └── benches/                # 性能基准测试
├── apps/
│   └── admin-web/              # 管理后台 (Vue 3)
├── docker/                     # Docker 部署
└── docs/                       # 文档
```

## 快速开始

### Docker 部署（推荐）

```bash
cd docker
docker compose up -d
```

首次访问 http://localhost:8081 进入初始化页面。

### 本地开发

#### 后端

```bash
cd plog-rs
cp config/default.toml config/config.toml
cargo run --release
```

#### 前端

```bash
cd apps/admin-web
npm install
npm run dev
```

## 服务端口

| 服务 | 端口 | 说明 |
|------|------|------|
| API | 8080 | RESTful API |
| 管理后台 | 8081 | Vue 3 SPA |
| 前台网站 | 8082 | 静态站点 |
| MySQL | 3306 | 数据库 |

## 功能特性

### 后台管理
- 文章、分类、标签、评论管理
- 用户角色与权限
- 主题与插件系统
- 系统设置

### 前台网站
- 文章浏览与搜索
- 分类筛选
- 多主题支持（明亮/暗色/护眼）
- 响应式设计

### 技术特性
- JWT 认证 (httpOnly Cookie)
- 优雅关闭与信号处理
- 请求超时保护
- 慢查询监控
- CI 强制 Lints 保护

## 性能优化

详见 [PERFORMANCE_TUNING.md](plog-rs/PERFORMANCE_TUNING.md)

### 基准测试

```bash
cd plog-rs
cargo bench --bench api_bench
```

### 性能分析

```bash
# CPU 火焰图
cargo flamegraph --root

# 慢查询监控
RUST_LOG=sqlx=debug cargo run
```

## 文档

| 文档 | 说明 |
|------|------|
| [DESIGN.md](DESIGN.md) | 架构设计 |
| [DEPLOYMENT.md](DEPLOYMENT.md) | 部署指南 |
| [plog-rs/README.md](plog-rs/README.md) | Rust 后端 |
| [plog-rs/PERFORMANCE.md](plog-rs/PERFORMANCE.md) | 性能优化 |
| [docker/README.md](docker/README.md) | Docker 部署 |

## API 概览

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/auth/login | 登录 |
| GET | /api/posts | 文章列表 |
| POST | /api/posts | 创建文章 |
| GET | /api/categories | 分类列表 |
| GET | /api/tags | 标签列表 |
| GET | /health | 健康检查 |

## 配置

### 环境变量

```bash
# 数据库
PLOG__DATABASE__URL=mysql://user:pass@host/db

# JWT
PLOG__AUTH__JWT_SECRET=your-secret

# 日志
RUST_LOG=plog_api=info,sqlx=debug
```

### 配置文件

编辑 `plog-rs/config/settings.toml`:

```toml
[database]
url = "mysql://plog:plog123@localhost/plog"
max_connections = 20

[server]
host = "0.0.0.0"
port = 8080

[auth]
jwt_secret = "your-secret-key"
jwt_expiration = 86400

[cors]
allowed_origins = ["http://localhost:8081"]
```

## 开发规范

### Rust 规范

项目遵循 Rust Architecture Guide V9.0.0:

- P0: 安全性优先 (无 unsafe, panic hook)
- P1: 可维护性 (sealed trait, non_exhaustive)
- P2: 工程效率 (workspace, CI lints)
- P3: 性能优化 (profiling-driven)

详见 [plog-rs/](plog-rs/) 目录下的各个模块。

## 致谢

本项目遵循 [Rust Coding Standards Skills](https://github.com/smile9493/Rust_Coding_Standards_Skills) 规范体系：

- **rust-architecture-guide** V9.0.0 — 通用工程宪法
- **rust-systems-cloud-infra-guide** V6.0.0 — 云基础设施规范
- **rust-wasm-frontend-infra-guide** V4.0.0 — WASM 前端规范

## License

MIT
