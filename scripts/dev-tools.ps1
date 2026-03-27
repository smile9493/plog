# Emlog Development Environment Management Script
param(
    [Parameter(Position=0)]
    [ValidateSet("start", "stop", "restart", "status", "logs", "sync", "backup", "clean", "install")]
    [string]$Action = "status",

    [Parameter(Position=1)]
    [string]$Service = "",

    [Parameter(Position=2)]
    [string]$Extra = ""
)

$ErrorActionPreference = "Stop"
$ComposeFile = "../config/docker-compose.dev.yml"

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "=== $Text ===" -ForegroundColor Green
    Write-Host ""
}

function Test-Docker {
    try {
        docker info | Out-Null
        return $true
    } catch {
        Write-Host "Docker is not running. Please start Docker Desktop first." -ForegroundColor Red
        return $false
    }
}

switch($Action) {
    "start" {
        Write-Header "Starting Development Environment"
        if (-not (Test-Docker)) { exit 1 }

        Write-Host "Starting all services..." -ForegroundColor Cyan
        docker compose -f $ComposeFile up -d

        Write-Host ""
        Write-Host "Waiting for services to start..." -ForegroundColor Yellow
        Start-Sleep -Seconds 5

        Write-Host ""
        Write-Host "Service Status:" -ForegroundColor Cyan
        docker compose -f $ComposeFile ps

        Write-Host ""
        Write-Host "Access URLs:" -ForegroundColor Green
        Write-Host "   Emlog Front:  http://localhost:18080" -ForegroundColor White
        Write-Host "   Emlog Admin:  http://localhost:18080/admin" -ForegroundColor White
        Write-Host "   phpMyAdmin:   http://localhost:18081" -ForegroundColor White
        Write-Host "   MailHog:      http://localhost:18026" -ForegroundColor White
    }

    "stop" {
        Write-Header "Stopping Development Environment"
        if (-not (Test-Docker)) { exit 1 }

        Write-Host "Stopping all services..." -ForegroundColor Cyan
        docker compose -f $ComposeFile down

        Write-Host "Services stopped." -ForegroundColor Green
    }

    "restart" {
        Write-Header "Restarting Development Environment"
        if (-not (Test-Docker)) { exit 1 }

        if ($Service) {
            Write-Host "Restarting service: $Service" -ForegroundColor Cyan
            docker compose -f $ComposeFile restart $Service
        } else {
            Write-Host "Restarting all services..." -ForegroundColor Cyan
            docker compose -f $ComposeFile restart
        }

        Write-Host "Services restarted." -ForegroundColor Green
    }

    "status" {
        Write-Header "Development Environment Status"
        if (-not (Test-Docker)) { exit 1 }

        docker compose -f $ComposeFile ps

        Write-Host ""
        Write-Host "Resource Usage:" -ForegroundColor Cyan
        docker stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}" | Select-String "emlog-dev"
    }

    "logs" {
        Write-Header "Viewing Logs"
        if (-not (Test-Docker)) { exit 1 }

        $targetService = if ($Service) { $Service } else { "emlog-dev" }
        $lines = if ($Extra) { $Extra } else { "100" }

        Write-Host "$targetService logs (last $lines lines):" -ForegroundColor Cyan
        docker compose -f $ComposeFile logs --tail $lines $targetService
    }

    "sync" {
        Write-Header "Syncing to Linux Test Environment"
        python scripts/sync_theme.py full
    }

    "backup" {
        Write-Header "Backing Up Database"
        if (-not (Test-Docker)) { exit 1 }

        $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
        $backupFile = "backup_$timestamp.sql"

        Write-Host "Backing up database to: $backupFile" -ForegroundColor Cyan
        docker exec emlog-dev-mysql mysqldump -u root -proot123456 emlog_dev > $backupFile

        Write-Host "Backup completed." -ForegroundColor Green
    }

    "clean" {
        Write-Header "Cleaning Development Environment"
        if (-not (Test-Docker)) { exit 1 }

        Write-Host "WARNING: This will delete all data!" -ForegroundColor Yellow
        $confirm = Read-Host "Confirm cleanup? (yes/no)"

        if ($confirm -eq "yes") {
            Write-Host "Stopping and removing containers and data..." -ForegroundColor Cyan
            docker compose -f $ComposeFile down -v
            Write-Host "Cleanup completed." -ForegroundColor Green
        } else {
            Write-Host "Cancelled." -ForegroundColor Red
        }
    }

    "install" {
        Write-Header "First-time Installation Guide"
        Write-Host "Installation Steps:" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "1. Start development environment:" -ForegroundColor Yellow
        Write-Host "   .\scripts\dev-tools.ps1 start" -ForegroundColor White
        Write-Host ""
        Write-Host "2. Visit Emlog installation page:" -ForegroundColor Yellow
        Write-Host "   http://localhost:18080" -ForegroundColor White
        Write-Host ""
        Write-Host "3. Fill in database information:" -ForegroundColor Yellow
        Write-Host "   Host:     mysql-dev" -ForegroundColor White
        Write-Host "   Database: emlog_dev" -ForegroundColor White
        Write-Host "   User:     emlog" -ForegroundColor White
        Write-Host "   Password: emlog123456" -ForegroundColor White
        Write-Host ""
        Write-Host "4. After installation, enable theme:" -ForegroundColor Yellow
        Write-Host "   Admin -> Appearance -> Templates -> Enable mytheme" -ForegroundColor White
        Write-Host ""
        Write-Host "5. Start developing!" -ForegroundColor Yellow
        Write-Host "   Edit files in content/templates/mytheme/" -ForegroundColor White
        Write-Host "   Refresh browser to see changes immediately" -ForegroundColor White
    }
}
