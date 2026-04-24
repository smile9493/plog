# Plog CMS

Plog 是一个基于 Rust 的内容管理系统，采用前后端分离架构，支持 Docker 部署与模块化扩展。

## 技术栈

- 后端：Rust、Axum、SeaORM、MySQL
- 前端：Vue 3、TypeScript、Element Plus
- 部署：Docker、Docker Compose、Nginx

## 项目结构

```text
plog/
├── apps/
│   ├── admin-web/       管理后台前端
│   ├── cli-rs/          命令行工具
│   └── installer-rs/    安装器
├── plog-rs/             Rust 后端工作区
│   ├── crates/          后端模块
│   ├── config/          配置文件
│   └── docs/            后端相关文档
├── docker/              Docker 与部署文档
├── docs/                总览、架构与审查类文档
└── content/             内容与资源目录
```

## 快速开始

### Docker 部署

```bash
cd docker
docker compose up -d
```

访问地址：
- 前台站点：`http://localhost`
- 管理后台：`http://localhost:8081`
- API 服务：`http://localhost:8080`

### 后端本地开发

```bash
cd plog-rs
cargo build
cargo run --bin plog-api
```

### 前端本地开发

```bash
cd apps/admin-web
npm install
npm run dev
```

## 文档索引

- `docs/README.md`：项目总览与文档入口
- `docs/architecture/README.md`：架构说明
- `docker/README.md`：Docker 部署说明
- `plog-rs/README.md`：Rust 后端开发说明
- `plog-rs/docs/API.md`：API 说明
- `plog-rs/docs/ASYNC_IO_MIGRATION_PLAN.md`：异步 IO 迁移计划

## 说明

当前仓库以 Linux 容器环境为主要运行目标，文档中的命令与路径已按该目标统一整理。
