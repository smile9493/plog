# Plog CMS

> 基于纯 Rust 架构的现代化内容管理系统

## 架构概览

```
┌─────────────────────────────────────────────────────────────────┐
│                            Nginx                                 │
│                      (反向代理 + SSL)                             │
└───────────────────────────┬─────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│   /api/*     │   │  /admin-web  │   │   /uploads   │
│  Rust API    │   │  Vue 3 SPA   │   │   静态资源    │
│   (8080)     │   │   (5173)     │   │              │
└──────┬───────┘   └──────────────┘   └──────────────┘
       │
       ▼
┌──────────────┐
│    MySQL     │
└──────────────┘
```

## 技术栈

| 层 | 技术 |
|---|------|
| 后端 | Rust (Axum + SeaORM + JWT + Argon2) |
| 前端 | Vue 3 + TypeScript + Element Plus |
| 数据库 | MySQL 8.0 |
| 服务器 | Nginx |
| 容器 | Docker |

## 目录结构

```
mytheme/
├── plog-rs/              # Rust 核心服务
│   ├── crates/
│   │   ├── contracts/    #   API 合约
│   │   ├── core/         #   核心类型
│   │   ├── auth/         #   认证 (JWT + Argon2)
│   │   ├── content/      #   内容管理 (SeaORM)
│   │   ├── settings/     #   设置管理
│   │   ├── media/        #   媒体管理
│   │   ├── audit/        #   审计日志
│   │   ├── plugin/       #   插件 Runtime
│   │   ├── theme/        #   主题 Runtime
│   │   └── api/          #   API 服务
│   ├── config/
│   │   └── nginx/        #   Nginx 配置
│   └── migrations/       #   数据库迁移
│
├── apps/                 # 应用
│   ├── admin-web/        #   管理后台 (Vue 3)
│   ├── cli-rs/           #   命令行工具
│   └── installer-rs/     #   安装器
│
├── content/              # 内容数据
│   ├── templates/        #   主题模板
│   ├── plugins/          #   插件
│   └── uploadfile/       #   上传文件
│
├── docker/               # Docker 配置
├── docs/                 # 文档
└── .trellis/             # 项目管理
```

## 快速开始

### 1. 构建 Rust 服务

```bash
cd plog-rs
cargo build --release
```

### 2. 配置数据库

```bash
cp plog-rs/config/default.toml plog-rs/config/local.toml
# 编辑 local.toml 配置数据库连接
```

### 3. 运行迁移

```bash
cd plog-rs
cargo run --bin migrate
```

### 4. 启动服务

```bash
./plog-rs/target/release/plog-api
```

### 5. 启动前端

```bash
cd apps/admin-web
npm install
npm run dev
```

## API 文档

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | `/api/auth/login` | 登录 |
| GET | `/api/posts` | 文章列表 |
| POST | `/api/posts` | 创建文章 |
| GET | `/api/categories` | 分类列表 |
| GET | `/api/tags` | 标签列表 |
| GET | `/api/comments` | 评论列表 |
| GET | `/api/settings` | 系统设置 |

详见: [docs/API.md](docs/API.md)

## 测试

```bash
cd plog-rs
cargo test
```

**测试覆盖**: 57 个测试全部通过 ✅

## Docker 部署

```bash
cd docker
docker compose up -d
```

## License

MIT
