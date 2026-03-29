# Plog CMS PHP 兼容层

## 概述

Phase 3 的 PHP 兼容层框架，将请求转发到 Rust API，保持旧版 API 兼容。

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│                  PHP Compat Layer                            │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐  │
│  │  PHP Entry  │────►│   Router    │────►│   Proxy     │  │
│  │  (入口)     │     │  (路由)     │     │  (转发)     │  │
│  └─────────────┘     └─────────────┘     └─────────────┘  │
│                                                     │        │
│                                                     ▼        │
│                                            ┌─────────────┐  │
│                                            │ Rust API    │  │
│                                            │ (8080)      │  │
│                                            └─────────────┘  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## 目录结构

```
compat/
├── index.php      # 入口文件
├── config.php     # 配置文件
├── Router.php     # 路由类
├── Proxy.php      # 代理类
├── Response.php   # 响应类
├── Logger.php     # 日志类
├── logs/          # 日志目录
└── README.md      # 本文档
```

## 路由映射

| v1 路由 | v2 路由 | 方法 |
|---------|---------|------|
| auth/login | /api/v2/auth/login | POST |
| auth/logout | /api/v2/auth/logout | POST |
| auth/user | /api/v2/auth/me | GET |
| posts | /api/v2/posts | GET/POST |
| posts/:id | /api/v2/posts/:id | GET/PUT/DELETE |
| categories | /api/v2/categories | GET/POST |
| categories/:id | /api/v2/categories/:id | GET/PUT/DELETE |
| tags | /api/v2/tags | GET/POST |
| tags/:id | /api/v2/tags/:id | GET/PUT/DELETE |
| comments | /api/v2/comments | GET/POST |
| comments/:id | /api/v2/comments/:id | GET/PUT/DELETE |

## 配置

编辑 `config.php` 修改配置：

```php
return [
    'rust_api' => [
        'host' => '127.0.0.1',
        'port' => 8080,
        'timeout' => 30,
        'retry' => 3,
    ],
    // ...
];
```

## 使用方式

### Nginx 配置

```nginx
# 将 /api/ 请求转发到 PHP 兼容层
location /api/ {
    fastcgi_pass php:9000;
    fastcgi_param SCRIPT_FILENAME /path/to/compat/index.php;
    include fastcgi_params;
}

# 将 /api/v2/ 请求直接转发到 Rust API
location /api/v2/ {
    proxy_pass http://127.0.0.1:8080;
}
```

### 直接访问

```
http://your-domain.com/api/posts
http://your-domain.com/api/categories
```

## 日志

日志文件位置：`logs/compat.log`

日志格式：
```
[2026-03-29 16:30:00] [INFO] Incoming request {"method":"GET","uri":"/api/posts"}
```

## 错误处理

所有错误返回统一格式：

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error description"
  }
}
```

## 版本信息

| 版本 | 状态 | 说明 |
|------|------|------|
| v1 | 维护中 | 旧版 API (通过兼容层访问) |
| v2 | 开发中 | 新版 Rust API |
