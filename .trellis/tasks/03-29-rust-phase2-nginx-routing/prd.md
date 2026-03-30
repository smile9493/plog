# Nginx 路由配置 - PRD

## 1. 概述

**任务名称**: Nginx 路由配置
**所属阶段**: Phase 2 - Rust 接管
**优先级**: P1 (重要)
**预计工时**: 1 周
**前置依赖**: Rust 内容管理 API 完成

## 2. 背景

Rust 服务上线后，需要配置 Nginx 进行路由分流，将不同请求转发到 Rust 或 PHP 服务。

## 3. 目标

1. 编写 Nginx 配置
2. 配置 SSL/TLS
3. 配置负载均衡
4. 编写部署脚本

## 4. 路由规则

```
/api/v2/*  → Rust Core Service (端口 8080)
/api/*     → PHP admin-api
/admin     → PHP 传统后台
/admin-web → 独立前端 (Vite 构建产物)
```

## 5. 验收标准

- [ ] Nginx 配置正确
- [ ] SSL/TLS 配置完成
- [ ] 路由分流正常
- [ ] 部署脚本可运行

## 6. 任务清单

### 6.1 Nginx 配置

- [ ] 编写基础配置
- [ ] 配置 Rust API 反向代理
- [ ] 配置 PHP FastCGI
- [ ] 配置静态文件服务
- [ ] 配置 CORS

### 6.2 SSL/TLS 配置

- [ ] 配置证书
- [ ] 配置 HTTPS 重定向
- [ ] 配置安全头

### 6.3 负载均衡

- [ ] 配置上游服务器
- [ ] 配置负载均衡策略
- [ ] 配置健康检查

### 6.4 部署脚本

- [ ] 编写部署脚本
- [ ] 编写回滚脚本
- [ ] 编写配置验证脚本

## 7. 交付物

1. Nginx 配置文件
2. SSL 证书配置
3. 部署脚本
4. 运维文档

## 8. Nginx 配置示例

```nginx
upstream rust_api {
    server 127.0.0.1:8080;
}

upstream php_api {
    server 127.0.0.1:9000;
}

server {
    listen 443 ssl;
    server_name admin.example.com;

    # Rust API (v2)
    location /api/v2/ {
        proxy_pass http://rust_api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # PHP API (v1)
    location /api/ {
        fastcgi_pass php_api;
        fastcgi_param SCRIPT_FILENAME $document_root/index.php;
        include fastcgi_params;
    }

    # 传统后台
    location /admin {
        fastcgi_pass php_api;
        fastcgi_param SCRIPT_FILENAME $document_root/admin/index.php;
        include fastcgi_params;
    }

    # 独立前端
    location /admin-web {
        alias /var/www/admin-web/dist;
        try_files $uri $uri/ /admin-web/index.html;
    }
}
```

## 9. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| 配置错误导致服务不可用 | 高 | 中 | 配置验证和测试 |
| SSL 证书过期 | 高 | 低 | 自动续期配置 |
| 性能瓶颈 | 中 | 中 | 监控和调优 |

## 10. 参考文档

- [design-direction.md](../../spec/plog-rust-migration/design-direction.md)
- [migration-plan.md](../../spec/plog-rust-migration/migration-plan.md)
- [boundaries-and-protocols.md](../../spec/plog-rust-migration/boundaries-and-protocols.md)
