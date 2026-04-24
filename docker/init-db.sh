#!/bin/bash
# =============================================================================
# Plog CMS 数据库初始化脚本
# 用于首次部署时创建默认管理员账户
# =============================================================================

set -e

# 配置
DEFAULT_USERNAME="admin"
DEFAULT_PASSWORD="admin123"
DEFAULT_NICKNAME="管理员"
DEFAULT_ROLE="admin"

# 数据库连接配置
DB_HOST="${DB_HOST:-mysql}"
DB_PORT="${DB_PORT:-3306}"
DB_NAME="${DB_NAME:-plog}"
DB_USER="${DB_USER:-plog}"
DB_PASSWORD="${DB_PASSWORD:-plog123}"

echo "=========================================="
echo "  Plog CMS 数据库初始化"
echo "=========================================="

# 等待 MySQL 就绪
echo "[1/4] 等待 MySQL 就绪..."
for i in {1..30}; do
    if mysqladmin ping -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASSWORD" --silent 2>/dev/null; then
        echo "      MySQL 已就绪"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "错误: MySQL 连接超时"
        exit 1
    fi
    sleep 1
done

# 检查用户表是否存在
echo "[2/4] 检查数据库状态..."
TABLE_EXISTS=$(mysql -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" -N -e "
    SELECT COUNT(*) FROM information_schema.tables
    WHERE table_schema='$DB_NAME' AND table_name='user';
" 2>/dev/null || echo "0")

if [ "$TABLE_EXISTS" = "0" ]; then
    echo "      用户表尚未创建，请等待 API 服务完成迁移..."
    echo "      将在 10 秒后重试..."
    sleep 10
    # 再次检查
    TABLE_EXISTS=$(mysql -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" -N -e "
        SELECT COUNT(*) FROM information_schema.tables
        WHERE table_schema='$DB_NAME' AND table_name='user';
    " 2>/dev/null || echo "0")
    
    if [ "$TABLE_EXISTS" = "0" ]; then
        echo "      用户表仍未创建，退出初始化服务。API 启动后将自动创建表结构。"
        echo "      请重启 db-init 服务以继续。"
        exit 0
    fi
    echo "      用户表已创建"
fi

# 检查是否已存在用户
echo "[3/4] 检查系统初始化状态..."
USER_COUNT=$(mysql -h"$DB_HOST" -P"$DB_PORT" -u"$DB_USER" -p"$DB_PASSWORD" "$DB_NAME" -N -e "
    SELECT COUNT(*) FROM user;
" 2>/dev/null || echo "0")

if [ "$USER_COUNT" != "0" ]; then
    echo "      系统已初始化 (存在 $USER_COUNT 个用户)"
    echo "=========================================="
    echo "初始化完成"
    echo "=========================================="
    exit 0
fi

echo "      系统尚未初始化"
echo ""
echo "[4/4] 准备网页初始化向导..."
echo ""
echo "=========================================="
echo "  数据库就绪!"
echo ""
echo "  请访问管理后台完成初始化:"
echo "  http://localhost:8081"
echo ""
echo "  您将在那里创建管理员账户"
echo "=========================================="
exit 0
