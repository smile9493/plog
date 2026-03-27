# Plog 开发环境搭建指南

## 前置要求

1. **安装 Docker Desktop for Windows**
   - 下载地址：https://www.docker.com/products/docker-desktop
   - 安装后启动 Docker Desktop
   - 确保 Docker Desktop 正在运行（系统托盘图标显示为运行状态）

2. **系统要求**
   - Windows 10/11 64位
   - 启用 WSL 2 或 Hyper-V
   - 至少 4GB 内存

## 快速启动

### 1. 启动 Docker Desktop
确保 Docker Desktop 正在运行，系统托盘图标显示为绿色。

### 2. 启动 Plog 开发环境
在项目目录下执行：
```bash
docker compose up -d
```

### 3. 访问 Plog
浏览器访问：http://localhost:18080

### 4. 首次安装配置
第一次访问时，填写数据库信息：
- **主机**：mysql
- **数据库名**：plog_dev
- **用户**：plog
- **密码**：plog123456

## 目录结构

```
F:\mytheme\
├── docker-compose.yml          # Docker Compose 配置文件
├── plog_data/                 # Plog 数据目录（自动创建）
├── content/
│   └── templates/
│       └── mytheme/            # 主题开发目录（已挂载）
│           ├── header.php
│           ├── footer.php
│           ├── log_list.php
│           ├── echo_log.php
│           ├── style.css
│           └── preview.jpg
└── README-Docker.md            # 本文档
```

## 开发流程

### 修改主题文件
1. 在 `content/templates/mytheme/` 目录下修改主题文件
2. 修改会立即生效，无需重启容器
3. 刷新浏览器查看效果

### 查看容器状态
```bash
docker compose ps
```

### 查看容器日志
```bash
docker compose logs plog
docker compose logs mysql
```

### 停止容器
```bash
docker compose down
```

### 重启容器
```bash
docker compose restart
```

## 常见问题

### 1. Docker Desktop 未运行
**错误信息**：`error during connect: Get "http://%2F%2F.%2Fpipe%2FdockerDesktopLinuxEngine`

**解决方法**：
- 启动 Docker Desktop
- 等待 Docker Desktop 完全启动（系统托盘图标变绿）
- 重新执行 `docker compose up -d`

### 2. 端口被占用
**错误信息**：`port is already allocated`

**解决方法**：
- 修改 `docker-compose.yml` 中的端口映射
- 例如改为 `"18081:80"`

### 3. 镜像拉取失败
**解决方法**：
- 检查网络连接
- 配置 Docker 镜像加速器
- 使用国内镜像源

### 4. 权限问题
**解决方法**：
- 确保 Docker Desktop 有访问项目目录的权限
- 在 Docker Desktop 设置中添加文件共享目录

## 数据库管理

### 连接数据库
使用 MySQL 客户端工具（如 Navicat、MySQL Workbench）连接：
- **主机**：localhost
- **端口**：3306（需要暴露端口）
- **用户**：plog
- **密码**：plog123456
- **数据库**：plog_dev

### 暴露 MySQL 端口（可选）
在 `docker-compose.yml` 的 mysql 服务下添加：
```yaml
ports:
  - "13306:3306"
```

## 生产环境部署

### 打包主题
将 `content/templates/mytheme` 目录打包为 zip 文件。

### 上传到生产环境
1. 登录 Plog 后台
2. 进入"外观" -> "模板管理"
3. 上传并启用主题

## 技术栈

- **Plog**：pro-latest-php7.4-apache
- **MySQL**：5.6
- **PHP**：7.4
- **Apache**：最新版

## 开发端口

- **Plog**：18080
- **MySQL**：不暴露（仅容器内部访问）

## 注意事项

1. **数据持久化**：数据存储在 Docker volume 中，删除容器不会丢失数据
2. **主题开发**：主题文件直接挂载，修改立即生效
3. **环境隔离**：此开发环境与生产环境完全隔离
4. **性能优化**：建议至少分配 2GB 内存给 Docker Desktop
