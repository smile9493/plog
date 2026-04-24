# Plog CMS 文档总览

这里是项目文档入口，适合先读这一页，再按主题进入更细的说明。

## 推荐阅读顺序

1. `README.md`：项目总览与快速开始
2. `docs/architecture/README.md`：架构与模块划分
3. `docker/README.md`：Docker 部署说明
4. `plog-rs/README.md`：Rust 后端开发说明
5. `plog-rs/docs/API.md`：API 接口说明
6. `docs/审查模板.md`：项目审查模板

## 文档分类

### 总览类
- `README.md`
- `docs/README.md`
- `docs/architecture/README.md`

### 后端类
- `plog-rs/README.md`
- `plog-rs/docs/API.md`
- `plog-rs/docs/ASYNC_IO_MIGRATION_PLAN.md`

### 部署类
- `docker/README.md`
- `docker/Dockerfile.api`
- `docker/docker-compose.yml`

### 评审与过程类
- `docs/审查模板.md`
- `.trellis/spec/**`
- `.trellis/tasks/**`

## 约定

- 文档中的路径以仓库根目录为基准。
- 运行环境以 Linux 容器为主，尽量避免 Windows 专属路径写法。
- 示例命令优先使用当前仓库实际结构，不再保留过时目录名。
