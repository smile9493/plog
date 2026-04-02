# Plog CMS - 现代化博客系统

## 项目简介

Plog 是一个基于 Rust 构建的现代化博客系统，采用前后端分离架构。

## 项目结构

```
mytheme/
├── apps/
│   ├── admin-web/          # Vue 3 管理后台
│   ├── cli-rs/             # Rust CLI 工具
│   └── installer-rs/       # Rust 安装器
├── plog-rs/                # Rust 后端 API
│   ├── crates/             # 工作空间成员
│   │   ├── api/            # API 路由
│   │   ├── core/           # 核心模块
│   │   ├── auth/           # 认证模块
│   │   ├── content/        # 内容管理
│   │   └── ...
│   ├── config/             # 配置文件
│   └── src/                # 主入口
├── content/
│   ├── templates/zen/      # Zen 前台主题
│   └── uploadfile/         # 上传文件
├── docker/                 # Docker 配置
└── docs/                   # 项目文档
```

## 技术栈

- **后端**: Rust + Axum + SeaORM + MySQL
- **前端**: Vue 3 + TypeScript + Element Plus
- **部署**: Docker + Docker Compose

## 快速开始

### 1. 启动服务
```bash
cd docker
docker compose up -d
```

### 2. 访问服务
- **前台展示**: http://localhost:8082
- **管理后台**: http://localhost:8081
- **API 服务**: http://localhost:8080

### 3. 编译后端
```bash
cd plog-rs
cargo build --release
```

### 4. 构建前端
```bash
cd apps/admin-web
npm install
npm run build
```

## 开发指南

### 后端开发
```bash
cd plog-rs
cargo run
```

### 前端开发
```bash
cd apps/admin-web
npm run dev
```

## 环境变量

创建 `docker/.env` 文件：
```
MYSQL_ROOT_PASSWORD=your_password
MYSQL_DATABASE=plog
MYSQL_USER=plog
MYSQL_PASSWORD=your_password
JWT_SECRET=your_jwt_secret
```

## 文档索引

- [API 文档](plog-rs/docs/API.md)
- [Docker 部署](docker/README.md)

## 许可证

MIT License
