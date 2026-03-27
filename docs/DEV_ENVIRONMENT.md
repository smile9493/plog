# Windows开发环境完整指南

## 🎯 架构说明

```
┌─────────────────────────────────────────────────────────┐
│                    Windows 开发环境                      │
├─────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  MySQL 5.7   │  │  Plog Pro   │  │  phpMyAdmin  │  │
│  │  Port: 13306 │  │  Port: 18080 │  │  Port: 18081 │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│         │                  │                           │
│         │                  ├─ 主题开发 (实时挂载)      │
│         │                  ├─ 插件开发 (实时挂载)      │
│         │                  └─ 日志查看 (实时挂载)      │
│         │                                               │
│  ┌──────────────┐                                       │
│  │   MailHog    │  邮件测试工具                         │
│  │  SMTP: 18025 │                                       │
│  │  Web: 18026  │                                       │
│  └──────────────┘                                       │
└─────────────────────────────────────────────────────────┘
                          │
                          │ 同步主题
                          ↓
┌─────────────────────────────────────────────────────────┐
│                 Linux 集成测试环境                       │
├─────────────────────────────────────────────────────────┤
│  ┌──────────────┐                                       │
│  │  Plog Pro   │  生产环境配置                         │
│  │  Port: 8080  │                                       │
│  └──────────────┘                                       │
└─────────────────────────────────────────────────────────┘
```

## 📦 服务组件

### 1. MySQL 数据库 (mysql-dev)
- **镜像**: mysql/mysql-server:5.7
- **端口**: 13306 (外部访问)
- **数据库**: plog_dev
- **用户**: plog / plog123456
- **Root**: root / root123456
- **用途**: 开发数据库,可直连管理

### 2. Plog 应用 (plog-dev)
- **镜像**: plog/plog:pro-latest-php7.4-apache
- **端口**: 18080
- **挂载目录**:
  - `./content/templates` → `/app/content/templates` (主题开发)
  - `./content/plugins` → `/app/content/plugins` (插件开发)
  - `./content/uploadfile` → `/app/content/uploadfile` (上传文件)
  - `./dev/logs` → `/app/logs` (日志查看)
- **特性**: 文件修改立即生效,无需重启

### 3. phpMyAdmin (数据库管理)
- **端口**: 18081
- **访问**: http://localhost:18081
- **用途**: 可视化管理数据库

### 4. MailHog (邮件测试)
- **SMTP端口**: 18025
- **Web界面**: http://localhost:18026
- **用途**: 捕获和查看邮件,无需真实SMTP服务器

## 🚀 快速启动

### 1. 启动开发环境
```bash
# 启动所有服务
docker compose -f docker-compose.dev.yml up -d

# 查看服务状态
docker compose -f docker-compose.dev.yml ps

# 查看日志
docker compose -f docker-compose.dev.yml logs -f plog-dev
```

### 2. 访问服务
- **Plog前台**: http://localhost:18080
- **Plog后台**: http://localhost:18080/admin
- **phpMyAdmin**: http://localhost:18081
- **MailHog**: http://localhost:18026

### 3. 首次安装配置
首次访问Plog时,填写数据库信息:
- **主机**: mysql-dev
- **数据库**: plog_dev
- **用户**: plog
- **密码**: plog123456

## 📁 目录结构

```
f:\mytheme\
├── docker-compose.dev.yml       # 开发环境配置
├── content/
│   ├── templates/               # 主题开发目录 (已挂载)
│   │   └── mytheme/
│   │       ├── header.php
│   │       ├── footer.php
│   │       ├── log_list.php
│   │       ├── echo_log.php
│   │       └── style.css
│   ├── plugins/                 # 插件开发目录 (已挂载)
│   └── uploadfile/              # 上传文件目录 (已挂载)
├── dev/
│   ├── mysql/
│   │   └── init/
│   │       └── 01-init.sql      # 数据库初始化脚本
│   ├── config/                  # 配置文件目录
│   └── logs/                    # 日志目录 (已挂载)
├── sync_theme.py                # 同步到Linux脚本
└── sync_config.json             # 同步配置
```

## 🔧 开发工作流

### 主题开发流程

1. **修改主题文件**
   ```bash
   # 编辑主题文件
   code content/templates/mytheme/header.php
   ```

2. **实时预览**
   - 保存文件后立即刷新浏览器
   - 无需重启容器
   - 支持热重载

3. **同步到Linux测试**
   ```bash
   # 同步到Linux集成测试环境
   python sync_theme.py full
   ```

4. **验证集成测试**
   - 访问: http://192.168.31.14:8080
   - 验证主题在生产环境的表现

### 插件开发流程

1. **创建插件目录**
   ```bash
   mkdir content/plugins/myplugin
   ```

2. **开发插件文件**
   ```bash
   code content/plugins/myplugin/myplugin.php
   ```

3. **在后台启用插件**
   - 访问后台 → 插件管理 → 启用插件

### 数据库管理

#### 方式1: phpMyAdmin
- 访问: http://localhost:18081
- 用户: root
- 密码: root123456

#### 方式2: MySQL客户端
```bash
# 连接数据库
mysql -h 127.0.0.1 -P 13306 -u root -p
# 密码: root123456
```

#### 方式3: Docker命令
```bash
# 进入MySQL容器
docker exec -it plog-dev-mysql mysql -u root -p
```

### 日志查看

#### 应用日志
```bash
# 查看Plog日志
tail -f dev/logs/plog.log

# 查看PHP错误日志
tail -f dev/logs/php_error.log
```

#### 容器日志
```bash
# 查看Plog容器日志
docker compose -f docker-compose.dev.yml logs -f plog-dev

# 查看MySQL容器日志
docker compose -f docker-compose.dev.yml logs -f mysql-dev
```

## 🛠️ 常用命令

### 服务管理
```bash
# 启动服务
docker compose -f docker-compose.dev.yml up -d

# 停止服务
docker compose -f docker-compose.dev.yml down

# 重启服务
docker compose -f docker-compose.dev.yml restart

# 重启单个服务
docker compose -f docker-compose.dev.yml restart plog-dev

# 查看服务状态
docker compose -f docker-compose.dev.yml ps

# 查看资源使用
docker stats plog-dev-mysql plog-dev-app
```

### 数据管理
```bash
# 备份数据库
docker exec plog-dev-mysql mysqldump -u root -proot123456 plog_dev > backup_$(date +%Y%m%d).sql

# 恢复数据库
docker exec -i plog-dev-mysql mysql -u root -proot123456 plog_dev < backup.sql

# 清理数据(谨慎!)
docker compose -f docker-compose.dev.yml down -v
```

### 开发调试
```bash
# 进入Plog容器
docker exec -it plog-dev-app bash

# 进入MySQL容器
docker exec -it plog-dev-mysql bash

# 查看PHP配置
docker exec plog-dev-app php -i

# 查看Apache配置
docker exec plog-dev-app apachectl -S
```

## 📧 邮件测试

### 配置Plog使用MailHog
在Plog后台设置邮件SMTP:
- **SMTP服务器**: mailhog
- **端口**: 1025
- **无需认证**

### 查看邮件
- 访问: http://localhost:18026
- 所有发送的邮件都会被捕获并显示

## 🔄 开发→测试→生产流程

### 完整工作流
```
1. Windows开发环境
   ↓ 修改主题/插件
   ↓ 实时预览 (localhost:18080)
   ↓
2. 本地测试通过
   ↓
3. 同步到Linux集成测试
   python sync_theme.py full
   ↓
4. Linux测试环境验证
   http://192.168.31.14:8080
   ↓
5. 测试通过
   ↓
6. 部署到生产环境
   (手动或自动化部署)
```

### 自动化同步脚本
```bash
#!/bin/bash
# dev-to-test.sh

echo "=== 开发环境 → 测试环境 ==="

# 1. 检查开发环境
echo "1. 检查开发环境..."
docker compose -f docker-compose.dev.yml ps

# 2. 同步主题
echo "2. 同步主题到Linux..."
python sync_theme.py full

# 3. 提示验证
echo "3. 请验证测试环境: http://192.168.31.14:8080"
echo "完成!"
```

## 🐛 故障排查

### 1. 容器无法启动
```bash
# 查看详细日志
docker compose -f docker-compose.dev.yml logs

# 检查端口占用
netstat -ano | findstr "18080"
netstat -ano | findstr "13306"
```

### 2. 文件修改不生效
- 检查挂载路径是否正确
- 重启容器: `docker compose -f docker-compose.dev.yml restart plog-dev`
- 清除浏览器缓存

### 3. 数据库连接失败
```bash
# 检查MySQL是否启动
docker compose -f docker-compose.dev.yml ps mysql-dev

# 查看MySQL日志
docker compose -f docker-compose.dev.yml logs mysql-dev

# 测试连接
docker exec -it plog-dev-mysql mysql -u plog -p
```

### 4. 权限问题
```bash
# 修复文件权限(Windows通常不需要)
# 如果遇到权限问题,检查Docker Desktop的文件共享设置
```

## 📊 性能优化

### 1. 资源限制
在docker-compose.dev.yml中添加资源限制:
```yaml
services:
  plog-dev:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
```

### 2. Docker Desktop设置
- 分配至少4GB内存
- 启用WSL 2后端
- 配置磁盘镜像位置

## 🔐 安全建议

1. **开发环境仅用于本地开发**
   - 不要暴露到公网
   - 使用随机端口(已配置)

2. **数据库密码**
   - 开发环境使用简单密码
   - 生产环境使用强密码

3. **定期备份**
   - 备份开发数据
   - 备份主题/插件代码

## 📝 开发技巧

### 1. VS Code集成
```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "php",
      "request": "launch",
      "name": "Listen for XDebug",
      "port": 9000,
      "pathMappings": {
        "/app": "${workspaceFolder}"
      }
    }
  ]
}
```

### 2. 热重载工具
- 使用Browser Sync自动刷新
- 配置文件监听

### 3. 代码质量
- 使用PHP CodeSniffer检查代码
- 使用ESLint检查JavaScript
- 使用StyleLint检查CSS

## 🎯 下一步

1. ✅ 启动开发环境
2. ✅ 开发主题/插件
3. ✅ 本地测试
4. ✅ 同步到Linux测试
5. ✅ 部署到生产环境

开发环境已配置完成,开始愉快地开发吧! 🚀
