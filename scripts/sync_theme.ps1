# Emlog Theme Sync Script
$ErrorActionPreference = "Stop"

Write-Host "=== Emlog Theme Sync Script ===" -ForegroundColor Green

# Step 1: Test SSH connection
Write-Host "`nStep 1: Testing SSH connection..." -ForegroundColor Yellow
try {
    $result = ssh -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@192.168.31.14 "echo 'OK'"
    if ($result -eq "OK") {
        Write-Host "SSH connection successful!" -ForegroundColor Green
    } else {
        throw "SSH connection failed"
    }
} catch {
    Write-Host "SSH key authentication not configured." -ForegroundColor Red
    Write-Host "`nPlease run these commands manually:" -ForegroundColor Yellow
    Write-Host "1. ssh root@192.168.31.14  (password: qqwwee)" -ForegroundColor Cyan
    Write-Host '2. mkdir -p ~/.ssh && chmod 700 ~/.ssh' -ForegroundColor Cyan
    Write-Host '3. echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILgwnqdAksrFJ57OLMPPIU4ty7dpYgyyzpD7EXeLUYQ1" >> ~/.ssh/authorized_keys' -ForegroundColor Cyan
    Write-Host '4. chmod 600 ~/.ssh/authorized_keys && exit' -ForegroundColor Cyan
    Write-Host "`nAfter configuration, run this script again." -ForegroundColor Yellow
    exit 1
}

# Step 2: Check emlog container
Write-Host "`nStep 2: Checking emlog container..." -ForegroundColor Yellow
$containerName = ssh root@192.168.31.14 "docker ps --filter 'name=emlog' --format '{{.Names}}' | head -1"
Write-Host "Found container: $containerName" -ForegroundColor Green

# Step 3: Get theme mount path
Write-Host "`nStep 3: Getting theme mount path..." -ForegroundColor Yellow
$themePath = ssh root@192.168.31.14 "docker inspect $containerName --format='{{range .Mounts}}{{if eq .Destination \"/app/content/templates\"}}{{.Source}}{{end}}{{end}}'"

if ($themePath) {
    Write-Host "Theme mount path: $themePath" -ForegroundColor Green
} else {
    $themePath = "/tmp/emlog_themes"
    Write-Host "Using default path: $themePath" -ForegroundColor Yellow
    ssh root@192.168.31.14 "mkdir -p $themePath"
}

# Step 4: Sync theme files
Write-Host "`nStep 4: Syncing theme files..." -ForegroundColor Yellow
$targetPath = "$themePath/mytheme"
Write-Host "Target: $targetPath" -ForegroundColor Cyan

# Create target directory
ssh root@192.168.31.14 "mkdir -p $targetPath"

# Sync files
scp -r "f:\mytheme\content\templates\mytheme\*" "root@192.168.31.14:$targetPath/"

if ($LASTEXITCODE -eq 0) {
    Write-Host "Theme files synced successfully!" -ForegroundColor Green
} else {
    Write-Host "Failed to sync theme files" -ForegroundColor Red
    exit 1
}

# Step 5: Verify
Write-Host "`nStep 5: Verifying theme files..." -ForegroundColor Yellow
$files = ssh root@192.168.31.14 "ls -la $targetPath"
Write-Host $files

Write-Host "`n=== Sync Complete ===" -ForegroundColor Green
Write-Host "Theme synced to: $targetPath" -ForegroundColor Cyan
