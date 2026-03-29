# Plog Rust 开发环境搭建

## 环境要求

- Rust 1.75+
- MySQL 8.0+
- Git

## 安装 Rust

```bash
# 安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Rust 工具链
rustup install stable
rustup default stable

# 安装组件
rustup component add rustfmt clippy
```

## 项目设置

```bash
# 克隆项目
cd plog-rs

# 安装依赖
cargo build

# 复制配置文件
cp config/default.toml config/development.toml

# 编辑配置
vim config/development.toml
```

## 数据库配置

```bash
# 创建数据库
mysql -u root -p
CREATE DATABASE plog CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'plog'@'localhost' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON plog.* TO 'plog'@'localhost';
FLUSH PRIVILEGES;
```

## 运行服务

```bash
# 开发模式运行
cargo run --bin server

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 项目结构

```
plog-rs/
├── Cargo.toml              # Workspace 配置
├── config/                 # 配置文件
│   ├── default.toml       # 默认配置
│   └── development.toml   # 开发配置
├── crates/
│   ├── core/              # 核心库
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs
│   │       ├── error.rs
│   │       └── types.rs
│   ├── auth/              # 认证模块
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── jwt.rs
│   │       └── password.rs
│   ├── content/           # 内容模块
│   │   └── src/
│   │       ├── lib.rs
│   │       └── entities.rs
│   └── api/               # API 服务
│       └── src/
│           ├── lib.rs
│           ├── routes/
│           └── handlers/
├── migrations/            # 数据库迁移
└── tests/                 # 集成测试
```

## API 端点

### 认证

- `POST /api/v1/auth/login` - 登录
- `POST /api/v1/auth/logout` - 登出
- `GET /api/v1/auth/user` - 获取当前用户

### 文章

- `GET /api/v1/posts` - 文章列表
- `GET /api/v1/posts/:id` - 文章详情
- `POST /api/v1/posts` - 创建文章
- `PUT /api/v1/posts/:id` - 更新文章
- `DELETE /api/v1/posts/:id` - 删除文章

### 分类

- `GET /api/v1/categories` - 分类列表

## 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| APP_ENV | 运行环境 | development |
| PLOG__DATABASE__URL | 数据库连接 | - |
| PLOG__AUTH__JWT_SECRET | JWT 密钥 | - |

## 故障排除

### 编译错误

```bash
# 清理构建缓存
cargo clean

# 更新依赖
cargo update
```

### 数据库连接错误

检查 `config/development.toml` 中的数据库配置是否正确。

## 相关文档

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Axum 文档](https://docs.rs/axum/)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/)
