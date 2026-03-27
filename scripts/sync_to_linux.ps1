# Emlog主题项目同步脚本
# 目标服务器: 192.168.31.14
# 用户: root
# 密码: qqwwee

$serverIP = "192.168.31.14"
$user = "root"
$password = "qqwwee"
$remotePath = "/root/mytheme"
$localPath = "f:\mytheme"

Write-Host "=== Emlog主题项目同步到Linux Docker ===" -ForegroundColor Green

# 方法1: 使用WinSCP (如果已安装)
Write-Host "`n请选择同步方式:" -ForegroundColor Yellow
Write-Host "1. 手动同步 (推荐) - 我会提供详细步骤"
Write-Host "2. 使用Git同步"
Write-Host "3. 打包后手动传输"

$choice = Read-Host "请输入选择 (1-3)"

switch($choice) {
    "1" {
        Write-Host "`n=== 手动同步步骤 ===" -ForegroundColor Cyan
        Write-Host "1. 打开新的PowerShell窗口,执行以下命令连接到Linux:"
        Write-Host "   ssh root@192.168.31.14" -ForegroundColor Green
        Write-Host "   密码: qqwwee" -ForegroundColor Green
        Write-Host "`n2. 连接成功后,在Linux上执行:"
        Write-Host "   mkdir -p /root/mytheme" -ForegroundColor Green
        Write-Host "   cd /root/mytheme" -ForegroundColor Green
        Write-Host "`n3. 在本地PowerShell执行以下命令同步文件:"
        Write-Host "   scp -r f:\mytheme\docker-compose.yml root@192.168.31.14:/root/mytheme/" -ForegroundColor Green
        Write-Host "   scp -r f:\mytheme\content root@192.168.31.14:/root/mytheme/" -ForegroundColor Green
        Write-Host "   scp -r f:\mytheme\emlog_data root@192.168.31.14:/root/mytheme/" -ForegroundColor Green
        Write-Host "`n4. 在Linux上启动Docker:"
        Write-Host "   cd /root/mytheme && docker compose up -d" -ForegroundColor Green
    }
    "2" {
        Write-Host "`n=== Git同步方式 ===" -ForegroundColor Cyan
        Write-Host "请确保项目已推送到Git仓库,然后在Linux上克隆"
    }
    "3" {
        Write-Host "`n=== 打包传输方式 ===" -ForegroundColor Cyan
        Write-Host "正在打包项目..."
        $archivePath = "f:\mytheme.tar.gz"
        # 使用7zip或tar打包
        Write-Host "打包完成: $archivePath"
        Write-Host "使用以下命令传输:"
        Write-Host "   scp f:\mytheme.tar.gz root@192.168.31.14:/tmp/" -ForegroundColor Green
    }
}

Write-Host "`n按任意键退出..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
