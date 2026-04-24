# Plog CMS 部署脚本说明

## 主要脚本

### `deploy.sh` - 统一部署脚本
这是主要的部署脚本，集成了所有功能。

**用法：**
```bash
# 完整部署（默认）
./deploy.sh

# 仅安装 Docker
./deploy.sh install

# 构建镜像
./deploy.sh build

# 启动服务
./deploy.sh up

# 停止服务
./deploy.sh down

# 重启服务
./deploy.sh restart

# 查看日志
./deploy.sh logs

# 查看状态
./deploy.sh status

# 清理资源
./deploy.sh clean

# 离线部署
./deploy.sh offline

# 显示帮助
./deploy.sh help
```

### `build-offline.sh` - 离线构建脚本
用于在本地构建所有 Docker 镜像并打包，然后传输到服务器进行离线部署。

**用法：**
```bash
./build-offline.sh
```

## 辅助脚本

### `docker/optimize-docker.sh` - Docker 优化脚本
优化 Docker 配置，包括镜像源、网络设置等。

**用法：**
```bash
sudo ./docker/optimize-docker.sh
```

### `docker/start.sh` - 快速启动脚本
Docker 服务的快速管理脚本。

**用法：**
```bash
cd docker
./start.sh [build|up|down|restart|logs|ps|clean|help]
```

### `fix_apt.sh` - APT 源修复脚本
修复 APT 源为国内镜像。

**用法：**
```bash
bash fix_apt.sh
```

### `plog-rs/config/nginx/deploy-nginx.sh` - Nginx 部署脚本
部署和配置 Nginx。

**用法：**
```bash
bash plog-rs/config/nginx/deploy-nginx.sh
```

## 部署流程

### 标准部署
1. 确保 Docker 已安装
2. 运行 `./deploy.sh` 进行完整部署

### 离线部署
1. 在网络好的环境运行 `./build-offline.sh`
2. 将生成的压缩包传输到服务器
3. 在服务器上解压并运行部署脚本

## 配置文件

- `docker/.env` - 环境变量配置
- `docker/docker-compose.yml` - Docker Compose 配置
- `docker/daemon.json` - Docker 守护进程配置

## 访问地址

部署完成后：
- 前台网站: http://localhost:8082
- 管理后台: http://localhost:8081
- API 服务: http://localhost:8080
- MySQL: localhost:3306

通过 Nginx 访问：
- 前台网站: http://localhost/
- 管理后台: http://localhost/admin/
- API 服务: http://localhost/api/