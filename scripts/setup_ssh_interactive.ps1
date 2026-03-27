# Interactive SSH Key Setup
Write-Host "=== SSH Key Setup for Linux Server ===" -ForegroundColor Green
Write-Host "Server: 192.168.31.14" -ForegroundColor Cyan
Write-Host "User: root" -ForegroundColor Cyan
Write-Host "Password: qqwwee" -ForegroundColor Cyan
Write-Host ""

Write-Host "Step 1: Connecting to Linux server..." -ForegroundColor Yellow
Write-Host "Please enter password when prompted: qqwwee" -ForegroundColor Gray
Write-Host ""

# Connect and setup SSH key
$commands = @(
    "mkdir -p ~/.ssh",
    "chmod 700 ~/.ssh",
    "echo 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILgwnqdAksrFJ57OLMPPIU4ty7dpYgyyzpD7EXeLUYQ1' >> ~/.ssh/authorized_keys",
    "chmod 600 ~/.ssh/authorized_keys",
    "echo 'SSH key configured successfully'",
    "cat ~/.ssh/authorized_keys"
)

$commandStr = $commands -join " && "
Write-Host "Executing: $commandStr" -ForegroundColor Gray
Write-Host ""

ssh root@192.168.31.14 $commandStr

Write-Host ""
Write-Host "Step 2: Testing SSH key authentication..." -ForegroundColor Yellow
$result = ssh -o StrictHostKeyChecking=no -o BatchMode=yes -o ConnectTimeout=10 root@192.168.31.14 "echo 'SSH_KEY_AUTH_SUCCESS'"

if ($result -eq "SSH_KEY_AUTH_SUCCESS") {
    Write-Host "SUCCESS: SSH key authentication is working!" -ForegroundColor Green
    Write-Host ""
    Write-Host "You can now run the sync script:" -ForegroundColor Yellow
    Write-Host "  powershell -ExecutionPolicy Bypass -File f:\mytheme\sync_theme.ps1" -ForegroundColor Cyan
} else {
    Write-Host "FAILED: SSH key authentication not working" -ForegroundColor Red
    Write-Host "Please check the configuration manually" -ForegroundColor Yellow
}
