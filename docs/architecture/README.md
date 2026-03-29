# Plog CMS 项目架构

## 概述

Plog CMS 是一个基于 **Rust + PHP 混合架构**的内容管理系统，采用渐进式迁移策略，从 PHP 逐步迁移到 Rust。

## 架构图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Plog CMS 架构                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                  │
│   │   Browser   │────►│   Nginx     │────►│  Frontend   │                  │
│   │   (客户端)   │     │  (反向代理)  │     │  (Vue 3)    │                  │
│   └─────────────┘     └─────────────┘     └─────────────┘                  │
│                              │                                                │
│              ┌───────────────┼───────────────┐                              │
│              ▼               ▼               ▼                              │
│     ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                      │
│     │  /api/v2/*  │  │  /api/*     │  │  /admin     │                      │
│     │  Rust API   │  │  PHP Compat │  │  PHP Legacy │                      │
│     │  (8080)     │  │  Layer      │  │             │                      │
│     └─────────────┘  └─────────────┘  └─────────────┘                      │
│              │               │                                               │
│              └───────────────┴───────────────┘                              │
│                              │                                                │
│                              ▼                                                │
│                    ┌─────────────────┐                                       │
│                    │    Database     │                                       │
│                    │    (MySQL)      │                                       │
│                    └─────────────────┘                                       │
│                                                                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 目录结构

```
mytheme/
├── apps/                          # 应用层
│   ├── admin-web/                 # 管理后台前端 (Vue 3)
│   │   ├── src/
│   │   │   ├── api/              # API 集成
│   │   │   ├── views/            # 页面组件
│   │   │   ├── components/       # 公共组件
│   │   │   ├── store/            # 状态管理 (Pinia)
│   │   │   ├── router/           # 路由配置
│   │   │   └── types/            # TypeScript 类型
│   │   └── package.json
│   └── admin-api/                 # 管理后台 API (PHP)
│
├── plog-rs/                       # Rust 微服务
│   ├── crates/
│   │   ├── api/                   # API 服务
│   │   │   ├── src/
│   │   │   │   ├── routes/       # 路由定义
│   │   │   │   │   ├── auth.rs   # 认证路由
│   │   │   │   │   ├── posts.rs  # 文章路由
│   │   │   │   │   ├── categories.rs
│   │   │   │   │   ├── tags.rs
│   │   │   │   │   └── comments.rs
│   │   │   │   └── lib.rs
│   │   │   └── Cargo.toml
│   │   ├── auth/                  # 认证模块
│   │   │   ├── src/
│   │   │   │   ├── jwt.rs        # JWT 服务
│   │   │   │   ├── password.rs   # 密码哈希
│   │   │   │   ├── middleware.rs # 认证中间件
│   │   │   │   └── service.rs    # 认证服务
│   │   │   └── tests/
│   │   ├── content/               # 内容模块
│   │   │   ├── src/
│   │   │   │   ├── entities/     # 数据实体
│   │   │   │   │   ├── user.rs
│   │   │   │   │   ├── post.rs
│   │   │   │   │   ├── category.rs
│   │   │   │   │   ├── tag.rs
│   │   │   │   │   └── comment.rs
│   │   │   │   └── repository/   # 数据访问层
│   │   │   │       ├── user.rs
│   │   │   │       ├── post.rs
│   │   │   │       ├── category.rs
│   │   │   │       ├── tag.rs
│   │   │   │       └── comment.rs
│   │   │   └── tests/
│   │   ├── core/                  # 核心模块
│   │   │   ├── src/
│   │   │   │   ├── config.rs     # 配置管理
│   │   │   │   ├── error.rs      # 错误处理
│   │   │   │   └── types.rs      # 通用类型
│   │   │   └── Cargo.toml
│   │   ├── plugin/                # 插件模块
│   │   └── theme/                 # 主题模块
│   ├── config/
│   │   ├── default.toml           # 默认配置
│   │   └── nginx/                 # Nginx 配置
│   │       ├── plog.conf          # 主配置
│   │       ├── ssl-params.conf    # SSL 配置
│   │       └── deploy-nginx.sh    # 部署脚本
│   ├── docs/
│   │   └── API.md                 # API 文档
│   ├── migrations/                # 数据库迁移
│   ├── Cargo.toml                 # Rust 项目配置
│   └── Cargo.lock
│
├── compat/                        # PHP 兼容层
│   ├── index.php                  # 入口文件
│   ├── config.php                 # 配置文件
│   ├── Proxy.php                  # 请求代理
│   ├── Router.php                 # 路由器
│   ├── Response.php               # 响应处理
│   ├── Logger.php                 # 日志系统
│   ├── plugin/                    # 插件兼容层
│   │   ├── PluginManager.php     # 插件管理器
│   │   ├── PluginLoader.php      # 插件加载器
│   │   ├── HookBridge.php        # Hook 桥接
│   │   └── PluginConfig.php      # 配置管理
│   └── theme/                     # 主题兼容层
│       ├── ThemeManager.php      # 主题管理器
│       ├── ThemeLoader.php       # 主题加载器
│       ├── Renderer.php          # 渲染器
│       └── TemplateEngine.php    # 模板引擎
│
├── content/                       # 内容目录
│   ├── templates/                 # 主题模板
│   │   ├── default/              # 默认主题
│   │   └── mytheme/              # 自定义主题
│   ├── plugins/                   # 插件
│   ├── uploadfile/               # 上传文件
│   ├── cache/                    # 缓存
│   └── languages/                # 语言文件
│
├── include/                       # PHP 核心库
│   ├── controller/               # 控制器
│   ├── model/                    # 数据模型
│   ├── service/                  # 服务层
│   └── lib/                      # 工具库
│
├── config/                        # 配置目录
├── docs/                          # 文档
└── .trellis/                      # 项目管理
    ├── tasks/                    # 任务管理
    ├── spec/                     # 规范文档
    └── workspace/                # 工作区
```

## 技术栈

### 后端 (Rust)
- **Web 框架**: Axum
- **ORM**: SeaORM
- **认证**: JWT + Argon2
- **数据库**: MySQL
- **异步运行时**: Tokio

### 后端 (PHP 兼容层)
- **版本**: PHP 8.0+
- **HTTP 客户端**: cURL
- **日志**: 自定义 Logger

### 前端
- **框架**: Vue 3 + TypeScript
- **构建工具**: Vite
- **UI 库**: Element Plus
- **状态管理**: Pinia
- **路由**: Vue Router

### 基础设施
- **Web 服务器**: Nginx
- **数据库**: MySQL
- **SSL**: Let's Encrypt

## API 设计

### v2 API (Rust)

| 端点 | 方法 | 说明 |
|------|------|------|
| `/api/v2/auth/login` | POST | 登录 |
| `/api/v2/auth/me` | GET | 获取当前用户 |
| `/api/v2/posts` | GET | 文章列表 |
| `/api/v2/posts/:id` | GET/PUT/DELETE | 文章 CRUD |
| `/api/v2/categories` | GET/POST | 分类管理 |
| `/api/v2/tags` | GET/POST | 标签管理 |
| `/api/v2/comments` | GET/POST | 评论管理 |

### 响应格式

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-29T20:00:00Z"
  }
}
```

## 数据库表结构

| 表名 | 说明 | 主键 |
|------|------|------|
| `user` | 用户表 | `uid` |
| `blog` | 文章表 | `gid` |
| `sort` | 分类表 | `sid` |
| `tag` | 标签表 | `tid` |
| `comment` | 评论表 | `cid` |

## 迁移路线图

```
Phase 1 ✅  API 合约 + 数据库层
    │
Phase 2 ✅  Rust 微服务 (auth, content-api, nginx)
    │
Phase 3 ✅  PHP 兼容层 (compat, plugin-compat, theme-compat)
    │
Phase 4 ✅  前端迁移 (admin-web 用户/内容管理)
    │
    ▼
[完成] 全部阶段完成
```

## 测试覆盖

| 模块 | 测试数量 | 状态 |
|------|---------|------|
| Auth | 10 | ✅ 通过 |
| Content | 9 | ✅ 通过 |
| **总计** | **19** | **✅ 通过** |

## 配置管理

### Rust 配置 (`plog-rs/config/default.toml`)
```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "mysql://..."

[auth]
jwt_secret = "..."
jwt_expiration = 3600
```

### PHP 兼容层配置 (`compat/config.php`)
```php
return [
    'rust_api' => [
        'host' => '127.0.0.1',
        'port' => 8080,
    ],
    'routes' => [...],
];
```

## 部署流程

1. 构建 Rust 服务
   ```bash
   cd plog-rs && cargo build --release
   ```

2. 配置 Nginx
   ```bash
   sudo ./config/nginx/deploy-nginx.sh install
   ```

3. 部署前端
   ```bash
   cd apps/admin-web && npm run build
   ```

4. 启动服务
   ```bash
   ./plog-rs/target/release/plog-api
   ```

## 会话记录

| Session | 日期 | 标题 |
|---------|------|------|
| 1 | 2026-03-29 | Phase2 Rust 微服务架构完成 |
| 2 | 2026-03-29 | Phase3 掏空 PHP 完成 |
| 3 | 2026-03-29 | Phase3 测试验证完成 |
| 4 | 2026-03-29 | Phase4 admin-web 用户管理完成 |
| 5 | 2026-03-29 | Phase4 admin-web 内容管理完成 |

---

**文档版本**: 1.0  
**最后更新**: 2026-03-29
