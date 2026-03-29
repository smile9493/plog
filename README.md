# Plog CMS

> 基于 Rust + PHP 混合架构的内容管理系统

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
│  /api/v2/*   │   │   /api/*     │   │   /admin     │
│  Rust Core   │   │  PHP Compat  │   │  PHP Legacy  │
│   (8080)     │   │    Layer     │   │              │
└──────┬───────┘   └──────┬───────┘   └──────────────┘
       │                  │
       └──────────────────┘
                │
        ┌───────▼───────┐
        │    MySQL      │
        └───────────────┘
```

## 目录结构

```
plog/
├── plog-rs/           # Rust 核心服务
│   ├── crates/
│   │   ├── api/       # API 服务 (Axum)
│   │   ├── auth/      # 认证 (JWT + Argon2)
│   │   ├── content/   # 内容管理 (SeaORM)
│   │   └── core/      # 核心类型
│   ├── config/
│   │   └── nginx/     # Nginx 配置
│   └── migrations/    # 数据库迁移
│
├── apps/              # 前端应用
│   └── admin-web/     # 管理后台 (Vue 3)
│
├── compat/            # PHP 兼容层
│   ├── index.php      # 入口
│   ├── Proxy.php      # 请求代理
│   ├── plugin/        # 插件兼容
│   └── theme/         # 主题兼容
│
├── content/           # 内容数据
│   ├── templates/     # 主题模板
│   ├── plugins/       # 插件
│   └── uploadfile/    # 上传文件
│
└── docs/              # 文档
```

## 技术栈

| 层 | 技术 |
|---|------|
| Rust 后端 | Axum + SeaORM + JWT |
| PHP 兼容 | cURL + 原生 PHP |
| 前端 | Vue 3 + Element Plus |
| 数据库 | MySQL |
| 服务器 | Nginx |

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

### 4. 配置 Nginx

```bash
sudo ./plog-rs/config/nginx/deploy-nginx.sh install
```

### 5. 启动服务

```bash
./plog-rs/target/release/plog-api
```

## API 文档

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | `/api/v2/auth/login` | 登录 |
| GET | `/api/v2/posts` | 文章列表 |
| POST | `/api/v2/posts` | 创建文章 |
| GET | `/api/v2/categories` | 分类列表 |
| GET | `/api/v2/tags` | 标签列表 |

详见: [docs/architecture/README.md](docs/architecture/README.md)

## 测试

```bash
cd plog-rs
cargo test
```

## 迁移进度

| 阶段 | 状态 | 说明 |
|------|------|------|
| Phase 1 | ✅ | API 合约 + 数据库层 |
| Phase 2 | ✅ | Rust 微服务 |
| Phase 3 | ✅ | PHP 兼容层 |
| Phase 4 | ✅ | 前端迁移 |

## License

MIT
