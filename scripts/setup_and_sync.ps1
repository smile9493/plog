# Emlog主题同步到Linux Docker - 完整配置脚本
# 目标: 192.168.31.14 (root/qqwwee)

Write-Host "=== Emlog主题同步配置脚本 ===" -ForegroundColor Green
Write-Host ""

# 步骤1: 配置SSH密钥
Write-Host "步骤1: 配置SSH密钥认证" -ForegroundColor Yellow
Write-Host "请在新的PowerShell窗口中执行以下命令:" -ForegroundColor Cyan
Write-Host ""
Write-Host "  ssh root@192.168.31.14" -ForegroundColor White
Write-Host "  密码: qqwwee" -ForegroundColor Gray
Write-Host ""
Write-Host "连接成功后,在Linux上执行:" -ForegroundColor Cyan
Write-Host '  mkdir -p ~/.ssh && chmod 700 ~/.ssh' -ForegroundColor White
Write-Host '  echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILgwnqdAksrFJ57OLMPPIU4ty7dpYgyyzpD7EXeLUYQ1 your_email@example.com" >> ~/.ssh/authorized_keys' -ForegroundColor White
Write-Host '  chmod 600 ~/.ssh/authorized_keys' -ForegroundColor White
Write-Host '  exit' -ForegroundColor White
Write-Host ""

Read-Host "按Enter键继续(完成SSH密钥配置后)"

# 步骤2: 测试SSH连接
Write-Host "`n步骤2: 测试SSH密钥连接" -ForegroundColor Yellow
$testResult = ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@192.168.31.14 "echo 'SSH连接成功' && docker ps --format 'table {{.Names}}\t{{.Status}}' | grep emlog" 2>&1

if ($testResult -match "SSH连接成功") {
    Write-Host "✓ SSH密钥认证配置成功!" -ForegroundColor Green
    Write-Host "`n当前emlog容器状态:" -ForegroundColor Cyan
    Write-Host $testResult
} else {
    Write-Host "✗ SSH连接失败,请检查密钥配置" -ForegroundColor Red
    Write-Host $testResult
    exit 1
}

# 步骤3: 检查emlog容器
Write-Host "`n步骤3: 检查emlog容器挂载目录" -ForegroundColor Yellow
$containerInfo = ssh root@192.168.31.14 "docker ps --filter 'name=emlog' --format '{{.Names}}' | head -1"

if ($containerInfo) {
    Write-Host "找到emlog容器: $containerInfo" -ForegroundColor Green

    # 获取主题目录挂载点
    $mountInfo = ssh root@192.168.31.14 "docker inspect $containerInfo --format='{{range .Mounts}}{{if eq .Destination \"/app/content/templates\"}}{{.Source}}{{end}}{{end}}'"

    if ($mountInfo) {
        Write-Host "主题挂载目录: $mountInfo" -ForegroundColor Cyan
        $targetPath = "$mountInfo/mytheme"
    } else {
        Write-Host "未找到主题挂载目录,使用默认路径" -ForegroundColor Yellow
        $targetPath = "/tmp/emlog_theme"
        ssh root@192.168.31.14 "mkdir -p $targetPath"
    }
} else {
    Write-Host "未找到emlog容器,创建临时目录" -ForegroundColor Yellow
    $targetPath = "/tmp/emlog_theme"
    ssh root@192.168.31.14 "mkdir -p $targetPath"
}

# 步骤4: 同步主题文件
Write-Host "`n步骤4: 同步主题文件到Linux" -ForegroundColor Yellow
Write-Host "目标路径: $targetPath" -ForegroundColor Cyan

# 同步主题文件
Write-Host "正在同步主题文件..." -ForegroundColor Gray
scp -r "f:\mytheme\content\templates\mytheme" "root@192.168.31.14:$targetPath/../"

if ($?) {
    Write-Host "✓ 主题文件同步成功!" -ForegroundColor Green
} else {
    Write-Host "✗ 主题文件同步失败" -ForegroundColor Red
    exit 1
}

# 步骤5: 验证同步结果
Write-Host "`n步骤5: 验证主题文件" -ForegroundColor Yellow
$verifyResult = ssh root@192.168.31.14 "ls -la $targetPath"

if ($verifyResult) {
    Write-Host "主题文件列表:" -ForegroundColor Cyan
    Write-Host $verifyResult
    Write-Host "`n✓ 主题同步完成!" -ForegroundColor Green
} else {
    Write-Host "✗ 验证失败" -ForegroundColor Red
}

Write-Host "`n=== 同步完成 ===" -ForegroundColor Green
Write-Host "主题已同步到: $targetPath" -ForegroundColor Cyan
Write-Host "请登录emlog后台启用主题" -ForegroundColor Yellow
