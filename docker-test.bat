@echo off
echo === Plog Monorepo 测试脚本 ===
echo.

echo 1. 检查依赖...
if not exist "vendor" (
    echo 安装依赖...
    docker run --rm -v "%cd%:/app" composer:2.0 install
)

echo.
echo 2. 运行单元测试...
docker run --rm -v "%cd%:/app" php:8.0-cli php /app/vendor/bin/phpunit --colors=always

echo.
echo 3. 代码风格检查...
docker run --rm -v "%cd%:/app" php:8.0-cli php /app/vendor/bin/phpcs --standard=PSR12 --colors /app/packages /app/apps

echo.
echo === 测试完成 ===
pause
