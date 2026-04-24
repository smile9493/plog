# 架构说明

Plog 采用前后端分离架构，后端 Rust 服务负责 API、认证、内容与扩展能力，前端负责管理界面和展示界面。

## 核心模块

- `plog-rs/crates/api`：HTTP API 与路由
- `plog-rs/crates/auth`：认证与授权
- `plog-rs/crates/content`：内容管理
- `plog-rs/crates/theme`：主题发现与渲染
- `plog-rs/crates/plugin`：插件发现与管理
- `plog-rs/crates/cache`：缓存能力

## 运行目标

- 主运行环境：Linux 容器
- 部署方式：Docker Compose
- 网关层：Nginx

## 文档维护原则

- 路径以仓库根目录为准
- 不再混用 Windows 本地路径写法
- 新增模块后同步更新本说明
