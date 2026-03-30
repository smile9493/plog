#!/bin/bash

# =============================================================================
# Plog CMS 快速启动脚本
# =============================================================================

set -e

echo "======================================"
echo "  Plog CMS Docker 部署脚本"
echo "======================================"
echo

# 检查 Docker 是否安装
if ! command -v docker &> /dev/null; then
    echo "错误: Docker 未安装"
    exit 1
fi

# 检查 Docker Compose 是否安装
if ! command -v docker &> /dev/null; then
    echo "错误: Docker Compose 未安装"
    exit 1
fi

# 检查 .env 文件
if [ ! -f .env ]; then
    echo "创建 .env 配置文件..."
    cp .env.example .env
    echo "请编辑 .env 文件修改配置"
    echo
fi

# 停止现有服务
echo "停止现有服务..."
docker compose down 2>/dev/null || true

# 构建镜像
echo "构建 Docker 镜像..."
docker compose build

# 启动服务
echo "启动服务..."
docker compose up -d

# 等待服务启动
echo "等待服务启动..."
sleep 10

# 检查服务状态
echo
echo "服务状态:"
docker compose ps

echo
echo "======================================"
echo "  部署完成！"
echo "======================================"
echo
echo "访问地址:"
echo "  前台网站:    http://localhost"
echo "  管理后台:    http://localhost/admin"
echo "  API 服务:    http://localhost/api"
echo "  健康检查:    http://localhost/health"
echo
echo "直接访问:"
echo "  前台网站:    http://localhost:8082"
echo "  管理后台:    http://localhost:8081"
echo "  API 服务:    http://localhost:8080"
echo
echo "查看日志: docker compose logs -f"
echo "停止服务: docker compose down"
echo
