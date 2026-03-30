@echo off
REM =============================================================================
REM Plog CMS 快速启动脚本 (Windows)
REM =============================================================================

echo ======================================
echo   Plog CMS Docker 部署脚本
echo ======================================
echo.

REM 检查 Docker 是否安装
docker --version >nul 2>&1
if errorlevel 1 (
    echo 错误: Docker 未安装
    exit /b 1
)

REM 检查 .env 文件
if not exist .env (
    echo 创建 .env 配置文件...
    copy .env.example .env
    echo 请编辑 .env 文件修改配置
    echo.
)

REM 停止现有服务
echo 停止现有服务...
docker compose down 2>nul

REM 构建镜像
echo 构建 Docker 镜像...
docker compose build

REM 启动服务
echo 启动服务...
docker compose up -d

REM 等待服务启动
echo 等待服务启动...
timeout /t 10 /nobreak >nul

REM 检查服务状态
echo.
echo 服务状态:
docker compose ps

echo.
echo ======================================
echo   部署完成！
echo ======================================
echo.
echo 访问地址:
echo   前台网站:    http://localhost
echo   管理后台:    http://localhost/admin
echo   API 服务:    http://localhost/api
echo   健康检查:    http://localhost/health
echo.
echo 直接访问:
echo   前台网站:    http://localhost:8082
echo   管理后台:    http://localhost:8081
echo   API 服务:    http://localhost:8080
echo.
echo 查看日志: docker compose logs -f
echo 停止服务: docker compose down
echo.

pause
