# Plog CMS

Plog 是一个基于 Rust 的现代博客系统，采用前后端分离架构，支持 Docker 一键部署与模块化扩展。

## 技术栈

- **后端**：Rust、Axum、SeaORM、MySQL
- **前端管理**：Vue 3、TypeScript、Element Plus、Pinia
- **前台网站**：纯 HTML/CSS/JS，零构建依赖，多主题支持
- **部署**：Docker、Docker Compose、Nginx

## 项目结构

```text
plog/
├── apps/
│   └── admin-web/          管理后台前端 (Vue 3 + Element Plus)
├── plog-rs/                Rust 后端工作区
│   ├── crates/
│   │   ├── api/            Axum Web API
│   │   ├── auth/           认证与密码模块
│   │   ├── content/        内容实体与仓储
│   │   └── core/           核心错误与工具
│   ├── config/             配置文件
│   └── migrations/         数据库迁移
├── docker/                 Docker 部署配置
├── content/
│   └── templates/zen/      前台主题模板
└── docs/                   文档
```

## 快速开始

### Docker 部署（推荐）

```bash
cd docker
docker compose up -d
```

首次部署访问 http://localhost:8081 进入初始化引导页面，创建管理员账户。

### 访问地址

| 服务 | 地址 | 说明 |
|------|------|------|
| 前台网站 | http://localhost:8082 | 博客前台，支持明亮/暗色/护眼主题 |
| 管理后台 | http://localhost:8081 | 文章、分类、标签、评论管理 |
| API 服务 | http://localhost:8080 | RESTful API |

### 默认账户

首次部署通过初始化页面创建，或使用已有账户登录。

### 后端本地开发

```bash
cd plog-rs
cp config/default.toml config/config.toml
cargo run --bin plog-api
```

### 前端本地开发

```bash
cd apps/admin-web
npm install
npm run dev
```

## 功能特性

### 后台管理
- 📝 文章管理 — 创建、编辑、发布、草稿
- 🏷️ 分类管理 — 分类创建、编辑、删除
- 🔖 标签管理 — 标签创建、编辑、删除
- 💬 评论管理 — 评论审核与回复
- 👤 用户管理 — 用户角色与权限
- ⚙️ 系统设置 — 基本设置、内容设置、SEO
- 🎨 主题管理 — 多主题切换（现代/明亮/暗色/绿色）
- 🔌 插件管理 — 插件安装与配置

### 前台网站
- 📖 文章列表 — 分页浏览，按分类筛选
- 📄 文章详情 — Markdown 渲染，代码高亮
- 📂 分类浏览 — 按主题浏览文章
- 🧭 导航收藏 — 精选开发资源与工具
- 📚 书库 — 阅读记录与推荐书单
- 🤖 MCP — Model Context Protocol 介绍与生态
- 🧩 Agent Skill — AI Agent 技能定义与能力扩展
- 🌓 主题切换 — 明亮/暗色/护眼，跨页面持久化

### 技术特性
- 🔒 JWT 认证 — httpOnly Cookie 安全存储
- 🐳 Docker 部署 — 一键启动，容器化运行
- 🌐 CORS 支持 — 跨域请求安全配置
- 📊 分页查询 — 后端分页，前端分页组件
- 🎯 响应式设计 — 适配桌面与移动端

## API 概览

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/auth/login | 登录 |
| POST | /api/auth/logout | 登出 |
| GET | /api/posts | 文章列表 |
| POST | /api/posts | 创建文章 |
| GET | /api/posts/:id | 文章详情 |
| PUT | /api/posts/:id | 更新文章 |
| DELETE | /api/posts/:id | 删除文章 |
| GET | /api/categories | 分类列表 |
| POST | /api/categories | 创建分类 |
| PUT | /api/categories/:id | 更新分类 |
| DELETE | /api/categories/:id | 删除分类 |
| GET | /api/tags | 标签列表 |
| POST | /api/tags | 创建标签 |
| PUT | /api/tags/:id | 更新标签 |
| DELETE | /api/tags/:id | 删除标签 |
| GET | /api/comments | 评论列表 |

## 文档索引

- `docs/` — 项目总览与架构文档
- `docker/` — Docker 部署说明
- `plog-rs/` — Rust 后端开发说明

## License

MIT
