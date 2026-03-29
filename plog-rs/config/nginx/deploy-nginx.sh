#!/bin/bash
# =============================================================================
# Plog CMS Nginx 部署脚本
# 
# 用法:
#   ./deploy-nginx.sh [命令] [选项]
#
# 命令:
#   install   - 安装 Nginx 配置
#   update    - 更新 Nginx 配置
#   rollback  - 回滚到上一版本
#   validate  - 验证配置文件
#   reload    - 重新加载配置
#   status    - 查看服务状态
# =============================================================================

set -e

# 配置
NGINX_CONF_DIR="/etc/nginx"
NGINX_CONF_FILE="plog.conf"
SSL_PARAMS_FILE="ssl-params.conf"
BACKUP_DIR="/etc/nginx/backup"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CONFIG_DIR="$PROJECT_DIR/config/nginx"
LOG_FILE="/var/log/plog-deploy.log"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 日志函数
log() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$LOG_FILE"
}

log_success() {
    log "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    log "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    log "${RED}[ERROR]${NC} $1"
}

# 检查 root 权限
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "请使用 root 权限运行此脚本"
        exit 1
    fi
}

# 创建备份目录
create_backup_dir() {
    if [ ! -d "$BACKUP_DIR" ]; then
        mkdir -p "$BACKUP_DIR"
        log "创建备份目录: $BACKUP_DIR"
    fi
}

# 备份当前配置
backup_config() {
    create_backup_dir
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local backup_file="$BACKUP_DIR/nginx_${timestamp}.tar.gz"
    
    if [ -f "$NGINX_CONF_DIR/conf.d/$NGINX_CONF_FILE" ]; then
        tar -czf "$backup_file" -C "$NGINX_CONF_DIR/conf.d" "$NGINX_CONF_FILE" 2>/dev/null || true
        log_success "配置已备份到: $backup_file"
        echo "$backup_file"
    fi
}

# 验证 Nginx 配置
validate_config() {
    log "验证 Nginx 配置..."
    if nginx -t 2>&1; then
        log_success "Nginx 配置验证通过"
        return 0
    else
        log_error "Nginx 配置验证失败"
        return 1
    fi
}

# 安装 Nginx 配置
install_config() {
    log "安装 Nginx 配置..."
    
    # 检查源文件
    if [ ! -f "$CONFIG_DIR/$NGINX_CONF_FILE" ]; then
        log_error "配置文件不存在: $CONFIG_DIR/$NGINX_CONF_FILE"
        exit 1
    fi
    
    # 复制主配置
    cp "$CONFIG_DIR/$NGINX_CONF_FILE" "$NGINX_CONF_DIR/conf.d/"
    log_success "已安装主配置: $NGINX_CONF_FILE"
    
    # 复制 SSL 配置
    if [ -f "$CONFIG_DIR/$SSL_PARAMS_FILE" ]; then
        mkdir -p "$NGINX_CONF_DIR/snippets"
        cp "$CONFIG_DIR/$SSL_PARAMS_FILE" "$NGINX_CONF_DIR/snippets/"
        log_success "已安装 SSL 配置: $SSL_PARAMS_FILE"
    fi
    
    # 验证配置
    if validate_config; then
        reload_nginx
        log_success "Nginx 配置安装完成"
    else
        log_error "配置验证失败，请检查配置文件"
        exit 1
    fi
}

# 更新 Nginx 配置
update_config() {
    log "更新 Nginx 配置..."
    
    # 备份当前配置
    local backup_file=$(backup_config)
    
    # 安装新配置
    install_config
    
    log_success "Nginx 配置更新完成"
}

# 回滚配置
rollback_config() {
    log "回滚 Nginx 配置..."
    
    # 查找最新的备份
    local latest_backup=$(ls -t "$BACKUP_DIR"/nginx_*.tar.gz 2>/dev/null | head -1)
    
    if [ -z "$latest_backup" ]; then
        log_error "没有找到备份文件"
        exit 1
    fi
    
    log "使用备份: $latest_backup"
    
    # 解压备份
    tar -xzf "$latest_backup" -C "$NGINX_CONF_DIR/conf.d/"
    
    # 验证配置
    if validate_config; then
        reload_nginx
        log_success "Nginx 配置已回滚"
    else
        log_error "回滚后配置验证失败"
        exit 1
    fi
}

# 重新加载 Nginx
reload_nginx() {
    log "重新加载 Nginx..."
    
    if systemctl reload nginx 2>/dev/null || service nginx reload 2>/dev/null; then
        log_success "Nginx 已重新加载"
    else
        log_error "Nginx 重新加载失败"
        exit 1
    fi
}

# 查看服务状态
show_status() {
    log "Nginx 服务状态:"
    
    if systemctl status nginx 2>/dev/null || service nginx status 2>/dev/null; then
        log_success "Nginx 正在运行"
    else
        log_warning "Nginx 未运行"
    fi
    
    echo ""
    log "当前配置文件:"
    ls -la "$NGINX_CONF_DIR/conf.d/" 2>/dev/null | grep -E "plog|nginx" || log "未找到配置文件"
    
    echo ""
    log "最近备份:"
    ls -lt "$BACKUP_DIR"/nginx_*.tar.gz 2>/dev/null | head -5 || log "没有备份文件"
}

# 主函数
main() {
    local command="${1:-help}"
    
    case "$command" in
        install)
            check_root
            install_config
            ;;
        update)
            check_root
            update_config
            ;;
        rollback)
            check_root
            rollback_config
            ;;
        validate)
            check_root
            validate_config
            ;;
        reload)
            check_root
            reload_nginx
            ;;
        status)
            show_status
            ;;
        help|--help|-h)
            echo "用法: $0 [命令]"
            echo ""
            echo "命令:"
            echo "  install   - 安装 Nginx 配置"
            echo "  update    - 更新 Nginx 配置"
            echo "  rollback  - 回滚到上一版本"
            echo "  validate  - 验证配置文件"
            echo "  reload    - 重新加载配置"
            echo "  status    - 查看服务状态"
            echo "  help      - 显示此帮助信息"
            ;;
        *)
            log_error "未知命令: $command"
            echo "使用 '$0 help' 查看帮助"
            exit 1
            ;;
    esac
}

# 执行主函数
main "$@"
