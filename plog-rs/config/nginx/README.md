# Nginx 配置说明

## 目录结构

```
config/nginx/
├── plog.conf          # 主配置文件
├── ssl-params.conf    # SSL 参数配置
├── deploy-nginx.sh    # 部署脚本
└── README.md          # 本文件
```

## 路由规则

| 路径 | 目标 | 说明 |
|------|------|------|
| `/api/v2/*` | Rust Core Service (8080) | 新版 API |
| `/api/*` | PHP (9000) | 旧版 API (兼容) |
| `/admin` | PHP (9000) | 传统后台 |
| `/admin-web` | 静态文件 | 独立前端 |

## 快速开始

### 1. 安装配置

```bash
# 使用部署脚本
sudo ./deploy-nginx.sh install

# 或手动复制
sudo cp config/nginx/plog.conf /etc/nginx/conf.d/
sudo cp config/nginx/ssl-params.conf /etc/nginx/snippets/
sudo nginx -t && sudo systemctl reload nginx
```

### 2. 验证配置

```bash
sudo ./deploy-nginx.sh validate
```

### 3. 更新配置

```bash
sudo ./deploy-nginx.sh update
```

### 4. 回滚配置

```bash
sudo ./deploy-nginx.sh rollback
```

### 5. 查看状态

```bash
sudo ./deploy-nginx.sh status
```

## 配置说明

### SSL/TLS

- 支持 TLSv1.2 和 TLSv1.3
- 使用 Mozilla 推荐的密码套件
- 启用 OCSP Stapling
- 配置 HSTS

### 反向代理

- 支持 WebSocket
- 配置超时和缓冲
- 传递客户端 IP 信息

### 静态资源

- 1 年缓存
- 支持 gzip 压缩

## 故障排除

### 检查配置语法

```bash
sudo nginx -t
```

### 查看错误日志

```bash
sudo tail -f /var/log/nginx/plog-error.log
```

### 查看访问日志

```bash
sudo tail -f /var/log/nginx/plog-access.log
```

## 注意事项

1. 部署前请先备份现有配置
2. 修改域名后需要更新 `server_name` 和 SSL 证书路径
3. 首次部署需要申请 SSL 证书：

```bash
sudo certbot certonly --webroot -w /var/www/certbot -d admin.example.com
```

4. 建议使用 `deploy-nginx.sh update` 而不是手动修改配置
