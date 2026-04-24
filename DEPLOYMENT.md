# Plog CMS 部署指南

本文档提供 Plog CMS 的完整部署说明，包括 Docker 部署和手动部署两种方式。

## 目录

- [系统要求](#系统要求)
- [Docker 部署（推荐）](#docker-部署推荐)
- [手动部署](#手动部署)
- [配置说明](#配置说明)
- [常见问题](#常见问题)

## 系统要求

### 最低配置

- **CPU**: 2 核
- **内存**: 2GB RAM
- **磁盘**: 10GB 可用空间
- **操作系统**: Ubuntu 20.04+ / Debian 11+ / CentOS 8+

### 推荐配置

- **CPU**: 4 核
- **内存**: 4GB RAM
- **磁盘**: 20GB SSD
- **操作系统**: Ubuntu 22.04 LTS

## Docker 部署（推荐）

### 快速部署

1. **克隆项目**

```bash
git clone <your-repo-url>
cd plog
```

2. **一键部署**

```bash
# 给脚本执行权限
chmod +x deploy.sh

# 执行部署
./deploy.sh
```

3. **手动部署**

```bash
cd docker

# 创建环境变量文件
cp .env.example .env

# 编辑配置（可选）
nano .env

# 启动服务
docker compose up -d

# 查看服务状态
docker compose ps
```

### 访问地址

| 服务 | 地址 | 说明 |
|------|------|------|
| 前台网站 | `http://localhost:8082` | 直接访问 |
| 管理后台 | `http://localhost:8081` | 直接访问 |
| API 服务 | `http://localhost:8080` | 直接访问 |
| Nginx 代理 | `http://localhost` | 统一入口 |
| MySQL | `localhost:3306` | 数据库连接 |

### Docker 常用命令

```bash
# 查看日志
docker compose logs -f

# 查看特定服务日志
docker compose logs -f api

# 重启服务
docker compose restart

# 停止服务
docker compose down

# 停止并删除数据卷
docker compose down -v

# 重新构建镜像
docker compose build

# 更新并重启
docker compose up -d --build
```

## 手动部署

### 1. 安装依赖

#### Ubuntu/Debian

```bash
# 更新系统
sudo apt update && sudo apt upgrade -y

# 安装基础工具
sudo apt install -y curl wget git build-essential

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 安装 Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# 安装 MySQL
sudo apt install -y mysql-server

# 安装 Nginx
sudo apt install -y nginx
```

#### CentOS/RHEL

```bash
# 更新系统
sudo yum update -y

# 安装基础工具
sudo yum install -y curl wget git gcc gcc-c++ make

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 安装 Node.js
curl -fsSL https://rpm.nodesource.com/setup_20.x | sudo bash -
sudo yum install -y nodejs

# 安装 MySQL
sudo yum install -y mysql-server

# 安装 Nginx
sudo yum install -y nginx
```

### 2. 配置数据库

```bash
# 启动 MySQL
sudo systemctl start mysql
sudo systemctl enable mysql

# 安全配置
sudo mysql_secure_installation

# 创建数据库和用户
sudo mysql -u root -p
```

```sql
CREATE DATABASE plog CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'plog'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON plog.* TO 'plog'@'localhost';
FLUSH PRIVILEGES;
EXIT;
```

### 3. 部署后端

```bash
# 进入后端目录
cd plog-rs

# 复制配置文件
cp config/default.toml config/local.toml

# 编辑配置
nano config/local.toml
```

修改数据库连接：

```toml
[database]
url = "mysql://plog:your_password@localhost:3306/plog"
```

```bash
# 构建项目
cargo build --release

# 运行数据库迁移
cargo run --bin plog-api -- migrate

# 启动服务（使用 systemd）
sudo cp ../deploy/plog-api.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl start plog-api
sudo systemctl enable plog-api
```

### 4. 部署管理后台

```bash
# 进入管理后台目录
cd ../apps/admin-web

# 安装依赖
npm install

# 构建生产版本
npm run build

# 配置 Nginx
sudo cp ../../deploy/nginx-admin.conf /etc/nginx/sites-available/plog-admin
sudo ln -s /etc/nginx/sites-available/plog-admin /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 5. 部署前台网站

```bash
# 进入前台目录
cd ../../content/templates/zen

# 复制到 Web 目录
sudo cp -r . /var/www/plog-frontend/

# 配置 Nginx
sudo cp ../../../deploy/nginx-frontend.conf /etc/nginx/sites-available/plog-frontend
sudo ln -s /etc/nginx/sites-available/plog-frontend /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## 配置说明

### 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `MYSQL_ROOT_PASSWORD` | MySQL root 密码 | `root123` |
| `MYSQL_DATABASE` | 数据库名 | `plog` |
| `MYSQL_USER` | 数据库用户 | `plog` |
| `MYSQL_PASSWORD` | 数据库密码 | `plog123` |
| `JWT_SECRET` | JWT 密钥 | 随机生成 |
| `RUST_LOG` | 日志级别 | `info` |

### API 配置

配置文件：`plog-rs/config/default.toml`

```toml
[app]
name = "Plog CMS"
env = "production"
debug = false

[database]
url = "mysql://plog:password@localhost:3306/plog"
max_connections = 20
min_connections = 5

[auth]
jwt_secret = "your-secret-key"
jwt_expiration = 86400

[server]
host = "0.0.0.0"
port = 8080
```

### Nginx 配置

主要配置文件：`docker/nginx.conf`

```nginx
upstream api {
    server api:8080;
}

upstream admin_web {
    server admin-web:80;
}

upstream frontend {
    server frontend:80;
}

server {
    listen 80;
    server_name your-domain.com;
    
    # API 代理
    location /api/ {
        proxy_pass http://api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
    
    # 管理后台
    location /admin/ {
        proxy_pass http://admin_web/;
    }
    
    # 前台网站
    location / {
        proxy_pass http://frontend;
    }
}
```

## 常见问题

### 1. 端口被占用

```bash
# 查看端口占用
sudo netstat -tlnp | grep :8080

# 修改端口
# Docker: 修改 docker-compose.yml 中的 ports
# 手动: 修改配置文件中的 port
```

### 2. 数据库连接失败

```bash
# 检查 MySQL 状态
sudo systemctl status mysql

# 检查用户权限
mysql -u plog -p -e "SHOW GRANTS FOR 'plog'@'localhost';"

# 重置权限
mysql -u root -p
GRANT ALL ON plog.* TO 'plog'@'localhost';
FLUSH PRIVILEGES;
```

### 3. API 服务无法启动

```bash
# 查看日志
docker compose logs api
# 或
sudo journalctl -u plog-api -f

# 检查配置
cat plog-rs/config/local.toml

# 手动运行测试
cargo run --bin plog-api
```

### 4. 前端页面无法加载

```bash
# 检查 Nginx 状态
sudo systemctl status nginx

# 检查配置语法
sudo nginx -t

# 查看错误日志
sudo tail -f /var/log/nginx/error.log
```

### 5. 文件上传失败

```bash
# 检查目录权限
ls -la /app/content/uploads

# 修改权限
sudo chown -R 1000:1000 /app/content/uploads
sudo chmod -R 755 /app/content/uploads

# 检查 Nginx 配置
client_max_body_size 100M;
```

## 生产环境建议

### 1. 安全配置

- 修改所有默认密码
- 使用强 JWT 密钥
- 启用 HTTPS
- 配置防火墙

```bash
# 配置防火墙
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 22/tcp
sudo ufw enable
```

### 2. 性能优化

- 启用 Nginx 缓存
- 配置 MySQL 查询缓存
- 使用 Redis 缓存（可选）

### 3. 监控和日志

```bash
# 查看服务状态
docker compose ps

# 查看资源使用
docker stats

# 日志轮转
sudo nano /etc/logrotate.d/plog
```

### 4. 备份策略

```bash
# 数据库备份
docker compose exec mysql mysqldump -u root -p plog > backup.sql

# 文件备份
tar -czf backup.tar.gz docker/volumes/

# 定时备份
0 2 * * * /path/to/backup.sh
```

## 技术支持

如有问题，请查看：

- [项目文档](docs/README.md)
- [API 文档](plog-rs/docs/API.md)
- [Docker 部署说明](docker/README.md)

## 更新日志

- v1.0.0: 初始版本
- 支持 Docker 一键部署
- 支持手动部署
- 完整的配置说明