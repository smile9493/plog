# Plog CMS Docker 部署说明

这里描述仓库当前的 Docker 运行方式，主要面向 Linux 容器环境。

## 启动方式

```bash
cd docker
docker compose up -d
```

## 服务端口

- API：`8080`
- 管理后台：`8081`
- 前台站点：`8082`
- MySQL：`3306`

## 目录说明

- `Dockerfile.api`：API 服务镜像构建
- `Dockerfile.admin-web`：管理后台镜像构建
- `Dockerfile.frontend`：前台站点镜像构建
- `docker-compose.yml`：整套服务编排

## 当前约定

- API 镜像采用多阶段构建，运行时使用 Linux slim 基础镜像。
- 文档中的路径均以仓库根目录为基准。
- 不再保留面向 Windows 本地路径的说明。

## 访问地址

- `http://localhost:8080`
- `http://localhost:8081`
- `http://localhost:8082`

如果使用 Nginx 统一入口，请以 `docker-compose.yml` 和 `nginx.conf` 为准。
