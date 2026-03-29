# Plog CMS 项目架构

> 版本: 2.0 | 最后更新: 2026-03-29

## 一、架构总览

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              请求流向                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│    Browser                                                                   │
│       │                                                                       │
│       ▼                                                                       │
│    ┌──────┐                                                                  │
│    │ Nginx│  (反向代理 + SSL)                                                 │
│    └──┬───┘                                                                  │
│       │                                                                       │
│       ├──► /api/v2/* ──► Rust Core (8080) ──► MySQL                          │
│       │                                                                       │
│       ├──► /api/*    ──► PHP Compat ──► Rust Core                            │
│       │                                                                       │
│       └──► /admin    ──► PHP Legacy                                          │
│                                                                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 二、核心目录 (只保留必要)

```
plog/
├── plog-rs/              # [Rust] 核心服务
│   ├── crates/           #     模块化 crates
│   │   ├── api/          #       API 服务 (Axum)
│   │   ├── auth/         #       认证 (JWT + Argon2)
│   │   ├── content/      #       内容管理 (SeaORM)
│   │   ├── core/         #       核心类型 + 配置
│   │   ├── plugin/       #       插件系统
│   │   └── theme/        #       主题系统
│   ├── config/           #     配置文件
│   │   ├── default.toml  #       应用配置
│   │   └── nginx/        #       Nginx 配置
│   ├── migrations/       #     数据库迁移 (SeaORM)
│   ├── docs/             #     API 文档
│   └── Cargo.toml        #     项目配置
│
├── apps/                 # [前端] 应用
│   └── admin-web/        #     管理后台 (Vue 3 + Element Plus)
│       ├── src/
│       │   ├── api/      #       API 集成
│       │   ├── views/    #       页面组件
│       │   ├── components#       公共组件
│       │   ├── store/    #       状态管理 (Pinia)
│       │   ├── router/   #       路由
│       │   └── types/    #       TypeScript 类型
│       └── package.json
│
├── compat/               # [PHP] 兼容层
│   ├── index.php         #     入口
│   ├── config.php        #     配置
│   ├── Proxy.php         #     请求代理
│   ├── Router.php        #     路由
│   ├── Response.php      #     响应
│   ├── plugin/           #     插件兼容
│   │   ├── PluginManager.php
│   │   ├── PluginLoader.php
│   │   ├── HookBridge.php
│   │   └── PluginConfig.php
│   └── theme/            #     主题兼容
│       ├── ThemeManager.php
│       ├── ThemeLoader.php
│       ├── Renderer.php
│       └── TemplateEngine.php
│
├── content/              # [数据] 内容目录
│   ├── templates/        #     主题模板
│   ├── plugins/          #     插件
│   ├── uploadfile/       #     上传文件
│   └── cache/            #     缓存
│
├── docs/                 # [文档] 项目文档
│   ├── architecture/     #     架构文档
│   └── api/              #     API 文档
│
└── .trellis/             # [管理] 项目管理
    ├── tasks/            #     任务管理
    ├── spec/             #     规范文档
    └── workspace/        #     工作区
```

## 三、模块边界

### 3.1 Rust 模块 (plog-rs/crates/)

| 模块 | 职责 | 依赖 | 不做 |
|------|------|------|------|
| **core** | 配置、错误、类型 | 无 | 业务逻辑 |
| **auth** | JWT、密码、中间件 | core, content | 数据存储 |
| **content** | 实体、Repository | core | API 路由 |
| **api** | 路由、Handler | auth, content | 数据访问 |
| **plugin** | 插件管理 | core | 插件实现 |
| **theme** | 主题管理 | core | 模板渲染 |

**依赖方向**: `api → auth → content → core`

### 3.2 前端模块 (apps/admin-web/src/)

| 模块 | 职责 | 说明 |
|------|------|------|
| **api/** | API 调用 | 封装 HTTP 请求 |
| **views/** | 页面组件 | 业务页面 |
| **components/** | 公共组件 | 可复用 UI |
| **store/** | 状态管理 | Pinia store |
| **router/** | 路由配置 | 页面导航 |
| **types/** | 类型定义 | TypeScript 类型 |

### 3.3 PHP 兼容层 (compat/)

| 模块 | 职责 | 说明 |
|------|------|------|
| **Proxy** | 请求转发 | PHP → Rust |
| **Router** | 路由映射 | v1 → v2 |
| **Response** | 响应处理 | 格式转换 |
| **plugin/** | 插件兼容 | Hook 桥接 |
| **theme/** | 主题兼容 | 模板适配 |

## 四、API 设计

### 4.1 端点列表

| 方法 | 端点 | 说明 | 模块 |
|------|------|------|------|
| POST | `/api/v2/auth/login` | 登录 | auth |
| GET | `/api/v2/auth/me` | 当前用户 | auth |
| GET | `/api/v2/posts` | 文章列表 | content |
| GET/PUT/DELETE | `/api/v2/posts/:id` | 文章 CRUD | content |
| GET/POST | `/api/v2/categories` | 分类管理 | content |
| GET/POST | `/api/v2/tags` | 标签管理 | content |
| GET/POST | `/api/v2/comments` | 评论管理 | content |

### 4.2 响应格式

```json
{
  "success": true,
  "data": {},
  "error": null,
  "meta": {
    "request_id": "uuid",
    "timestamp": "2026-03-29T20:00:00Z"
  }
}
```

## 五、数据模型

### 5.1 数据库表

| 表名 | 主键 | 说明 |
|------|------|------|
| `user` | `uid` | 用户 |
| `blog` | `gid` | 文章 |
| `sort` | `sid` | 分类 |
| `tag` | `tid` | 标签 |
| `comment` | `cid` | 评论 |

### 5.2 Rust 实体映射

```rust
// plog-rs/crates/content/src/entities/
user.rs      → user::Model { uid, username, password, ... }
post.rs      → post::Model { gid, title, content, ... }
category.rs  → category::Model { sid, sortname, ... }
tag.rs       → tag::Model { tid, tagname, ... }
comment.rs   → comment::Model { cid, gid, content, ... }
```

## 六、迁移路线

```
Phase 1 ✅ API 合约 + 数据库层
         └─ 实体定义, Repository, 类型系统

Phase 2 ✅ Rust 微服务
         ├─ auth: JWT + Argon2 + 中间件
         ├─ content-api: CRUD 完整实现
         └─ nginx: 反向代理配置

Phase 3 ✅ PHP 兼容层
         ├─ compat: 请求转发代理
         ├─ plugin-compat: Hook 桥接
         └─ theme-compat: 模板适配

Phase 4 ✅ 前端迁移
         ├─ admin-web/users: 用户管理
         └─ admin-web/content: 内容管理
```

## 七、测试覆盖

| 模块 | 文件 | 测试数 | 状态 |
|------|------|--------|------|
| Auth | `crates/auth/tests/` | 10 | ✅ |
| Content | `crates/content/tests/` | 9 | ✅ |
| **总计** | | **19** | **✅** |

## 八、部署

### 8.1 构建

```bash
# Rust
cd plog-rs && cargo build --release

# 前端
cd apps/admin-web && npm run build
```

### 8.2 配置

```bash
# Nginx
sudo ./config/nginx/deploy-nginx.sh install
```

### 8.3 启动

```bash
./plog-rs/target/release/plog-api
```

## 九、技术栈

| 层 | 技术 |
|---|------|
| **Rust 后端** | Axum + SeaORM + JWT + Argon2 + Tokio |
| **PHP 兼容** | cURL + 原生 PHP |
| **前端** | Vue 3 + Vite + Element Plus + Pinia |
| **数据库** | MySQL |
| **Web 服务器** | Nginx |

---

*文档路径: docs/architecture/README.md*
