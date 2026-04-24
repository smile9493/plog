# Plog Rust 后端开发说明

本目录是 Rust 后端工作区，主要提供 API、认证、内容、主题、插件、缓存等能力。

## 环境要求

- Rust stable
- MySQL 8.0
- Linux / Linux 容器环境优先

## 常用命令

```bash
cargo build
cargo run --bin plog-api
cargo test
cargo fmt
cargo clippy
```

## 目录说明

```text
plog-rs/
├── Cargo.toml
├── crates/
│   ├── api/
│   ├── auth/
│   ├── cache/
│   ├── content/
│   ├── core/
│   ├── plugin/
│   └── theme/
├── config/
└── docs/
```

## API 入口

- 服务入口：`crates/api/src/lib.rs`
- 路由入口：`crates/api/src/routes/mod.rs`
- 错误响应：`crates/api/src/error.rs`
- 响应封装：`crates/api/src/response.rs`

## 说明

- 旧版示例里出现的 `/api/v1/...` 路径已不再作为主文档参考。
- 以当前 `crates/api/src/routes/` 下的实现为准。
- 如果你在容器里运行，优先查看 `docker/README.md`。
