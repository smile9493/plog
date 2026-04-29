# Plog CMS Rust Backend

> 微内核架构的 Rust 后端服务

## 架构

```
plog-rs/
├── packages/           # 基础包
│   ├── plog-core/      # 核心错误与配置
│   └── plog-shared/    # 共享类型与 trait
├── modules/            # 功能模块
│   ├── content/        # 内容实体与仓储
│   ├── auth/           # JWT 认证与密码
│   ├── settings/       # 系统设置
│   ├── media/          # 媒体上传
│   └── audit/          # 审计日志
├── extensions/         # 扩展
│   ├── plugin/         # 插件管理器
│   └── theme/          # 主题管理器
├── plog-api/           # API 服务入口
├── crates/             # 独立模块 (带测试)
├── migrations/         # 数据库迁移
├── config/             # 配置文件
└── benches/            # 性能基准
```

## 环境要求

- Rust 1.88+
- MySQL 8.0
- Linux 容器环境优先

## 常用命令

```bash
# 开发
cargo run

# 生产
cargo run --release

# 测试
cargo test
cargo clippy

# 基准测试
cargo bench --bench api_bench
```

## 模块说明

| 模块 | 说明 |
|------|------|
| packages/plog-core | 核心错误与配置 |
| packages/plog-shared | 共享类型、trait、API 响应 |
| modules/content | 文章、分类、标签、评论 |
| modules/auth | JWT、密码、中间件 |
| plog-api | Axum API 入口 |

## 配置

编辑 `config/settings.toml`:

```toml
[database]
url = "mysql://plog:plog123@localhost/plog"
max_connections = 20

[server]
host = "0.0.0.0"
port = 8080

[auth]
jwt_secret = "your-secret"
jwt_expiration = 86400
```

## 性能优化

详见 [PERFORMANCE.md](PERFORMANCE.md) 和 [PERFORMANCE_TUNING.md](PERFORMANCE_TUNING.md)

### 慢查询监控

```bash
RUST_LOG=sqlx=debug cargo run
```

## 开发规范

### CI 强制 Lints

```rust
#![deny(clippy::await_holding_lock)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::todo)]
```

### 规范优先级

| 级别 | 说明 |
|------|------|
| P0 | 安全性: 无 unsafe, panic hook |
| P1 | 可维护性: sealed trait |
| P2 | 工程效率: workspace |
| P3 | 性能: profiling |

## License

MIT
