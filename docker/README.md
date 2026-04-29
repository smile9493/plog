# Plog CMS Docker 部署

> 完整的容器化部署方案

## 快速启动

```bash
cd docker
docker compose up -d
```

首次访问 http://localhost:8081 进入初始化页面。

## 服务架构

| 服务 | 端口 | 说明 |
|------|------|------|
| API | 8080 | Rust 后端 |
| 管理后台 | 8081 | Vue 3 SPA |
| 前台网站 | 8082 | 静态站点 |
| MySQL | 3306 | 数据库 |
| Nginx | 80/443 | 反向代理 |

## 环境变量

编辑 `.env`:

```bash
# 数据库
MYSQL_ROOT_PASSWORD=root123
MYSQL_DATABASE=plog
MYSQL_USER=plog
MYSQL_PASSWORD=plog123

# JWT
JWT_SECRET=your-secret-key

# 性能监控 (慢查询)
RUST_LOG=plog_api=info,sqlx=debug
```

## 常用命令

```bash
# 启动
docker compose up -d

# 停止
docker compose down

# 重启
docker compose restart api

# 重建
docker compose build api && docker compose up -d api

# 日志
docker compose logs -f api

# 状态
docker compose ps
```

## 性能监控

### 慢查询

```bash
docker compose logs api | grep -i slow
```

### 资源

```bash
docker stats plog-api plog-mysql
```

## 故障排查

| 问题 | 解决 |
|------|------|
| API 连接失败 | 检查 MySQL 健康状态 |
| 内存不足 | 调整容器资源限制 |
| 慢查询多 | 查看 sqlx 日志，添加索引 |

## 文档

- [../README.md](../README.md) - 项目总览
- [../plog-rs/PERFORMANCE.md](../plog-rs/PERFORMANCE.md) - 性能优化
