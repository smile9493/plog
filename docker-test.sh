#!/bin/bash

echo "=== Plog Monorepo 测试脚本 ==="
echo ""

# 检查 Docker 是否运行
if ! docker info > /dev/null 2>&1; then
    echo "错误: Docker 未运行"
    exit 1
fi

echo "1. 检查依赖..."
if [ ! -d "vendor" ]; then
    echo "安装依赖..."
    docker run --rm -v "$(pwd):/app" composer:2.0 install
fi

echo ""
echo "2. 运行单元测试..."
docker run --rm -v "$(pwd):/app" php:8.0-cli php /app/vendor/bin/phpunit --colors=always

echo ""
echo "3. 代码风格检查..."
docker run --rm -v "$(pwd):/app" php:8.0-cli php /app/vendor/bin/phpcs --standard=PSR12 --colors /app/packages /app/apps

echo ""
echo "=== 测试完成 ==="
