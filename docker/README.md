# Plog CMS Docker 部署指南

## 快速开始

### 1. 配置环境变量

```bash
cd docker
cp .env.example .env
# 编辑 .env 文件，修改密码和密钥
```

### 2. 构建并启动服务

```bash
# 构建所有服务
docker compose build

# 启动所有服务
docker compose up -d

# 查看服务状态
docker compose ps

# 查看日志
docker compose logs -f
```

### 3. 访问服务

- **前台网站**: http://localhost
- **管理后台**: http://localhost/admin
- **API 服务**: http://localhost/api
- **健康检查**: http://localhost/health

### 4. 直接访问各服务端口

- **前台网站**: http://localhost:8082
- **管理后台**: http://localhost:8081
- **API 服务**: http://localhost:8080
- **MySQL**: localhost:3306

## 服务架构

```
┌─────────────────────────────────────────────────────────────┐
│                        Nginx (80/443)                        │
│                      反向代理 + SSL                          │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   /api/*     │ │   /admin/*   │ │     /*       │
│  API 服务    │ │  管理后台    │ │  前台网站    │
│   (8080)     │ │   (8081)     │ │   (8082)     │
└──────┬───────┘ └──────────────┘ └──────────────┘
       │
       ▼
┌──────────────┐
│    MySQL     │
│   (3306)     │
└──────────────┘
```

## 服务说明

### 1. MySQL 数据库
- 镜像: mysql:8.0
- 端口: 3306
- 数据持久化: mysql_data

### 2. API 服务 (Rust)
- 镜像: 自定义构建
- 端口: 8080
- 技术栈: Rust + Axum + SeaORM
- 功能: 提供后端 API 接口

### 3. 管理后台 (Vue 3)
- 镜像: 自定义构建
- 端口: 8081
- 技术栈: Vue 3 + TypeScript + Element Plus
- 功能: 管理员后台管理界面

### 4. 前台网站
- 镜像: 自定义构建
- 端口: 8082
- 技术栈: Nginx + 静态文件
- 功能: 用户访问的展示网站

### 5. Nginx 反向代理
- 镜像: nginx:alpine
- 端口: 80, 443
- 功能: 路由分发、负载均衡、SSL

## 常用命令

```bash
# 停止所有服务
docker compose down

# 重启服务
docker compose restart

# 查看特定服务日志
docker compose logs api
docker compose logs admin-web
docker compose logs frontend

# 进入容器
docker compose exec api bash
docker compose exec mysql mysql -u root -p

# 清理所有数据
docker compose down -v
```

## 数据备份

```bash
# 备份 MySQL 数据
docker compose exec mysql mysqldump -u root -p plog > backup.sql

# 恢复 MySQL 数据
docker compose exec -T mysql mysql -u root -p plog < backup.sql
```

## 生产环境建议

1. **修改默认密码**: 修改 .env 中的所有密码
2. **配置 SSL**: 使用 Let's Encrypt 或自签名证书
3. **限制端口暴露**: 仅暴露必要的端口
4. **监控日志**: 定期检查日志文件
5. **定期备份**: 设置自动备份任务

## 故障排查

### 服务无法启动
```bash
# 查看详细日志
docker compose logs --tail=100 api

# 检查容器状态
docker compose ps

# 检查网络
docker network ls
```

### 数据库连接失败
```bash
# 检查 MySQL 是否运行
docker compose ps mysql

# 检查数据库连接
docker compose exec mysql mysql -u plog -p
```

### 前端无法访问
```bash
# 检查 Nginx 配置
docker compose exec nginx nginx -t

# 检查前端服务
docker compose logs frontend
```
