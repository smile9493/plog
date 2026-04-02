# Plog CMS

> 基于 Rust 构建的现代化内容管理系统

## 技术栈

| 层 | 技术 |
|---|------|
| 后端 | Rust (Axum + SeaORM + JWT) |
| 前端 | Vue 3 + TypeScript + Element Plus |
| 数据库 | MySQL 8.0 |
| 部署 | Docker + Docker Compose |

## 目录结构

```
mytheme/
├── plog-rs/              # Rust 后端服务
│   ├── crates/
│   │   ├── api/          #   API 路由
│   │   ├── core/         #   核心模块
│   │   ├── auth/         #   认证模块
│   │   └── content/      #   内容管理
│   └── config/           #   配置文件
│
├── apps/
│   ├── admin-web/        # 管理后台 (Vue 3)
│   └── cli-rs/           # 命令行工具
│
├── content/
│   └── templates/zen/    # 前台主题
│
├── docker/               # Docker 配置
└── docs/                 # 文档
```

## 快速开始

### Docker 部署（推荐）

```bash
cd docker
docker compose up -d
```

访问：
- 前台展示：http://localhost:8082
- 管理后台：http://localhost:8081
- API 服务：http://localhost:8080

### 本地开发

**后端**
```bash
cd plog-rs
cargo build --release
./target/release/plog-api
```

**前端**
```bash
cd apps/admin-web
npm install
npm run dev
```

## API 端点

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | `/api/auth/login` | 登录 |
| GET | `/api/posts` | 文章列表 |
| POST | `/api/posts` | 创建文章 |
| GET | `/api/categories` | 分类列表 |
| GET | `/api/tags` | 标签列表 |
| GET | `/api/comments` | 评论列表 |

## License

MIT
