#!/bin/bash
# Emlog开发环境管理脚本 (Bash版本)

set -e

COMPOSE_FILE="docker-compose.dev.yml"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

function print_header {
    echo ""
    echo -e "${GREEN}=== $1 ===${NC}"
    echo ""
}

function test_docker {
    if ! docker info &> /dev/null; then
        echo -e "${RED}❌ Docker未运行,请先启动Docker Desktop${NC}"
        exit 1
    fi
}

function show_help {
    echo "Emlog开发环境管理工具"
    echo ""
    echo "用法: ./dev-tools.sh <命令> [参数]"
    echo ""
    echo "命令:"
    echo "  start       启动开发环境"
    echo "  stop        停止开发环境"
    echo "  restart     重启开发环境 [服务名]"
    echo "  status      查看服务状态"
    echo "  logs        查看日志 [服务名] [行数]"
    echo "  sync        同步到Linux测试环境"
    echo "  backup      备份数据库"
    echo "  clean       清理开发环境(删除数据)"
    echo "  install     显示安装指南"
    echo ""
    echo "示例:"
    echo "  ./dev-tools.sh start"
    echo "  ./dev-tools.sh logs emlog-dev 200"
    echo "  ./dev-tools.sh restart mysql-dev"
}

case "${1:-help}" in
    start)
        print_header "启动开发环境"
        test_docker

        echo -e "${CYAN}🚀 启动所有服务...${NC}"
        docker compose -f $COMPOSE_FILE up -d

        echo ""
        echo -e "${YELLOW}⏳ 等待服务启动...${NC}"
        sleep 5

        echo ""
        echo -e "${CYAN}📊 服务状态:${NC}"
        docker compose -f $COMPOSE_FILE ps

        echo ""
        echo -e "${GREEN}🌐 访问地址:${NC}"
        echo "   Emlog前台:  http://localhost:18080"
        echo "   Emlog后台:  http://localhost:18080/admin"
        echo "   phpMyAdmin: http://localhost:18081"
        echo "   MailHog:    http://localhost:18026"
        ;;

    stop)
        print_header "停止开发环境"
        test_docker

        echo -e "${CYAN}🛑 停止所有服务...${NC}"
        docker compose -f $COMPOSE_FILE down

        echo -e "${GREEN}✅ 服务已停止${NC}"
        ;;

    restart)
        print_header "重启开发环境"
        test_docker

        if [ -n "$2" ]; then
            echo -e "${CYAN}🔄 重启服务: $2${NC}"
            docker compose -f $COMPOSE_FILE restart $2
        else
            echo -e "${CYAN}🔄 重启所有服务...${NC}"
            docker compose -f $COMPOSE_FILE restart
        fi

        echo -e "${GREEN}✅ 服务已重启${NC}"
        ;;

    status)
        print_header "开发环境状态"
        test_docker

        docker compose -f $COMPOSE_FILE ps

        echo ""
        echo -e "${CYAN}📊 资源使用:${NC}"
        docker stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}" | grep "emlog-dev" || true
        ;;

    logs)
        print_header "查看日志"
        test_docker

        SERVICE="${2:-emlog-dev}"
        LINES="${3:-100}"

        echo -e "${CYAN}📋 $SERVICE 日志(最近$LINES行):${NC}"
        docker compose -f $COMPOSE_FILE logs --tail $LINES $SERVICE
        ;;

    sync)
        print_header "同步到Linux测试环境"
        python sync_theme.py full
        ;;

    backup)
        print_header "备份数据库"
        test_docker

        TIMESTAMP=$(date +%Y%m%d_%H%M%S)
        BACKUP_FILE="backup_$TIMESTAMP.sql"

        echo -e "${CYAN}💾 备份数据库到: $BACKUP_FILE${NC}"
        docker exec emlog-dev-mysql mysqldump -u root -proot123456 emlog_dev > $BACKUP_FILE

        echo -e "${GREEN}✅ 备份完成${NC}"
        ;;

    clean)
        print_header "清理开发环境"
        test_docker

        echo -e "${YELLOW}⚠️  这将删除所有数据!${NC}"
        read -p "确认清理? (yes/no): " confirm

        if [ "$confirm" = "yes" ]; then
            echo -e "${CYAN}🗑️  停止并删除容器和数据...${NC}"
            docker compose -f $COMPOSE_FILE down -v
            echo -e "${GREEN}✅ 清理完成${NC}"
        else
            echo -e "${RED}❌ 已取消${NC}"
        fi
        ;;

    install)
        print_header "首次安装指南"
        echo -e "${CYAN}📝 首次安装步骤:${NC}"
        echo ""
        echo -e "${YELLOW}1. 启动开发环境:${NC}"
        echo "   ./dev-tools.sh start"
        echo ""
        echo -e "${YELLOW}2. 访问Emlog安装页面:${NC}"
        echo "   http://localhost:18080"
        echo ""
        echo -e "${YELLOW}3. 填写数据库信息:${NC}"
        echo "   主机:     mysql-dev"
        echo "   数据库:   emlog_dev"
        echo "   用户:     emlog"
        echo "   密码:     emlog123456"
        echo ""
        echo -e "${YELLOW}4. 完成安装后,启用主题:${NC}"
        echo "   后台 → 外观 → 模板管理 → 启用 mytheme"
        echo ""
        echo -e "${YELLOW}5. 开始开发!${NC}"
        echo "   修改 content/templates/mytheme/ 下的文件"
        echo "   保存后立即刷新浏览器查看效果"
        ;;

    help|--help|-h)
        show_help
        ;;

    *)
        echo -e "${RED}❌ 未知命令: $1${NC}"
        echo ""
        show_help
        exit 1
        ;;
esac
