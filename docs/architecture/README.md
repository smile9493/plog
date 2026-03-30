# Plog CMS 项目架构

> 版本: 3.0 | 最后更新: 2026-03-30

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
│       ├──► /api/*       ──► Rust API (8080) ──► MySQL                        │
│       │                                                                       │
│       ├──► /admin-web   ──► Vue 3 SPA (5173)                                 │
│       │                                                                       │
│       └──► /uploads     ──► 静态资源                                          │
│                                                                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 二、核心目录

```
mytheme/
├── plog-rs/              # Rust 核心服务
│   ├── crates/
│   │   ├── contracts/    #   API 合约定义
│   │   ├── core/         #   核心类型 + 配置
│   │   ├── auth/         #   认证 (JWT + Argon2)
│   │   ├── content/      #   内容管理 (SeaORM)
│   │   ├── settings/     #   设置管理
│   │   ├── media/        #   媒体管理
│   │   ├── audit/        #   审计日志
│   │   ├── plugin/       #   插件 Runtime
│   │   ├── theme/        #   主题 Runtime
│   │   └── api/          #   API 服务 (Axum)
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

## 三、技术栈

| 层 | 技术 | 说明 |
|---|------|------|
| 后端 | Rust | Axum + SeaORM + JWT + Argon2 |
| 前端 | Vue 3 | TypeScript + Element Plus |
| 数据库 | MySQL 8.0 | 主数据库 |
| 服务器 | Nginx | 反向代理 + SSL |
| 容器 | Docker | 部署方案 |

## 四、模块依赖

```
contracts ← core
auth ← core, content
content ← core
settings ← core, contracts
media ← core, contracts
audit ← core, contracts
plugin ← core, contracts
theme ← core, contracts
api ← core, auth, content
```

**依赖方向正确，无循环依赖**

## 五、API 端点

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | `/api/auth/login` | 登录 |
| POST | `/api/auth/logout` | 登出 |
| GET | `/api/auth/me` | 当前用户 |
| GET | `/api/posts` | 文章列表 |
| POST | `/api/posts` | 创建文章 |
| GET | `/api/posts/:id` | 文章详情 |
| PUT | `/api/posts/:id` | 更新文章 |
| DELETE | `/api/posts/:id` | 删除文章 |
| GET | `/api/categories` | 分类列表 |
| GET | `/api/tags` | 标签列表 |
| GET | `/api/comments` | 评论列表 |
| GET | `/api/settings` | 系统设置 |

## 六、测试覆盖

| 模块 | 测试数 | 状态 |
|------|--------|------|
| contracts | 11 | ✅ |
| auth | 10 | ✅ |
| content | 9 | ✅ |
| settings | 5 | ✅ |
| media | 6 | ✅ |
| audit | 7 | ✅ |
| plugin | 9 | ✅ |
| theme | 10 | ✅ |
| **总计** | **67** | **✅** |

## 七、代码统计

| 类型 | 数量 |
|------|------|
| Rust 文件 | 65 |
| 代码行数 | 7,433 |
| Git 提交 | 30 |

---

**文档版本**: 3.0  
**最后更新**: 2026-03-30
