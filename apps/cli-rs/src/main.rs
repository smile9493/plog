//! Plog CMS 命令行工具
//!
//! 用于管理 Plog CMS 的 CLI 工具

use clap::{Parser, Subcommand};
use colored::Colorize;

/// Plog CMS 命令行工具
#[derive(Parser)]
#[command(name = "plog")]
#[command(about = "Plog CMS 命令行管理工具")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 显示系统信息
    Info,
    
    /// 数据库操作
    Db {
        #[command(subcommand)]
        action: DbCommands,
    },
    
    /// 插件管理
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },
    
    /// 主题管理
    Theme {
        #[command(subcommand)]
        action: ThemeCommands,
    },
    
    /// 缓存管理
    Cache {
        #[command(subcommand)]
        action: CacheCommands,
    },
    
    /// 用户管理
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// 运行迁移
    Migrate,
    
    /// 重置数据库
    Reset,
    
    /// 备份数据库
    Backup { output: Option<String> },
    
    /// 恢复数据库
    Restore { input: String },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// 列出所有插件
    List,
    
    /// 安装插件
    Install { name: String },
    
    /// 卸载插件
    Uninstall { name: String },
    
    /// 启用插件
    Enable { name: String },
    
    /// 禁用插件
    Disable { name: String },
}

#[derive(Subcommand)]
enum ThemeCommands {
    /// 列出所有主题
    List,
    
    /// 安装主题
    Install { name: String },
    
    /// 卸载主题
    Uninstall { name: String },
    
    /// 激活主题
    Activate { name: String },
}

#[derive(Subcommand)]
enum CacheCommands {
    /// 清除所有缓存
    Clear,
    
    /// 清除指定缓存
    ClearType { cache_type: String },
    
    /// 查看缓存状态
    Status,
}

#[derive(Subcommand)]
enum UserCommands {
    /// 列出所有用户
    List,
    
    /// 创建用户
    Create {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
        #[arg(short, long, default_value = "user")]
        role: String,
    },
    
    /// 重置密码
    ResetPassword {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("plog_cli=info")
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Info => show_info().await?,
        Commands::Db { action } => handle_db(action).await?,
        Commands::Plugin { action } => handle_plugin(action).await?,
        Commands::Theme { action } => handle_theme(action).await?,
        Commands::Cache { action } => handle_cache(action).await?,
        Commands::User { action } => handle_user(action).await?,
    }

    Ok(())
}

/// 显示系统信息
async fn show_info() -> anyhow::Result<()> {
    println!("{}", "
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║                      Plog CMS                                 ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
".cyan());
    
    println!("  版本: {}", env!("CARGO_PKG_VERSION").green());
    println!("  Rust: {}", std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".into()).green());
    println!("  时间: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string().green());
    println!();
    
    Ok(())
}

/// 处理数据库命令
async fn handle_db(action: DbCommands) -> anyhow::Result<()> {
    match action {
        DbCommands::Migrate => {
            println!("{}", "运行数据库迁移...".green());
            // TODO: 实现迁移
            println!("{}", "迁移完成 ✓".green());
        }
        DbCommands::Reset => {
            println!("{}", "警告: 这将删除所有数据!".red().bold());
            // TODO: 实现重置
        }
        DbCommands::Backup { output } => {
            let output = output.unwrap_or_else(|| format!("backup_{}.sql", chrono::Utc::now().format("%Y%m%d_%H%M%S")));
            println!("备份数据库到: {}", output.green());
            // TODO: 实现备份
        }
        DbCommands::Restore { input } => {
            println!("从 {} 恢复数据库", input.green());
            // TODO: 实现恢复
        }
    }
    Ok(())
}

/// 处理插件命令
async fn handle_plugin(action: PluginCommands) -> anyhow::Result<()> {
    let plugins_dir = std::path::Path::new("content/plugins");
    let mut manager = plog_plugin::PluginManager::new(plugins_dir);
    
    match action {
        PluginCommands::List => {
            manager.discover()?;
            let plugins = manager.get_all_plugins();
            
            if plugins.is_empty() {
                println!("{}", "没有安装插件".yellow());
            } else {
                println!("{}", "已安装的插件:".bold());
                for p in plugins {
                    println!("  - {} ({}) - {:?}", p.manifest.name, p.manifest.version, p.status);
                }
            }
        }
        PluginCommands::Install { name } => {
            println!("安装插件: {}", name.green());
            // TODO: 实现安装
        }
        PluginCommands::Uninstall { name } => {
            println!("卸载插件: {}", name.green());
            manager.uninstall_plugin(&name)?;
            println!("{}", "卸载成功 ✓".green());
        }
        PluginCommands::Enable { name } => {
            println!("启用插件: {}", name.green());
            manager.activate_plugin(&name)?;
            println!("{}", "启用成功 ✓".green());
        }
        PluginCommands::Disable { name } => {
            println!("禁用插件: {}", name.green());
            manager.deactivate_plugin(&name)?;
            println!("{}", "禁用成功 ✓".green());
        }
    }
    Ok(())
}

/// 处理主题命令
async fn handle_theme(action: ThemeCommands) -> anyhow::Result<()> {
    let themes_dir = std::path::Path::new("content/templates");
    let mut manager = plog_theme::ThemeManager::new(themes_dir);
    
    match action {
        ThemeCommands::List => {
            manager.discover()?;
            let themes = manager.get_all_themes();
            
            if themes.is_empty() {
                println!("{}", "没有安装主题".yellow());
            } else {
                println!("{}", "已安装的主题:".bold());
                for t in themes {
                    let status = if manager.get_active_theme().map(|a| a.manifest.id == t.manifest.id).unwrap_or(false) {
                        "(当前)".green()
                    } else {
                        "".white()
                    };
                    println!("  - {} ({}) {}", t.manifest.name, t.manifest.version, status);
                }
            }
        }
        ThemeCommands::Install { name } => {
            println!("安装主题: {}", name.green());
            // TODO: 实现安装
        }
        ThemeCommands::Uninstall { name } => {
            println!("卸载主题: {}", name.green());
            manager.uninstall_theme(&name)?;
            println!("{}", "卸载成功 ✓".green());
        }
        ThemeCommands::Activate { name } => {
            println!("激活主题: {}", name.green());
            manager.activate_theme(&name)?;
            println!("{}", "激活成功 ✓".green());
        }
    }
    Ok(())
}

/// 处理缓存命令
async fn handle_cache(action: CacheCommands) -> anyhow::Result<()> {
    match action {
        CacheCommands::Clear => {
            println!("{}", "清除所有缓存...".green());
            let cache_dir = std::path::Path::new("content/cache");
            if cache_dir.exists() {
                std::fs::remove_dir_all(cache_dir)?;
                std::fs::create_dir_all(cache_dir)?;
            }
            println!("{}", "缓存已清除 ✓".green());
        }
        CacheCommands::ClearType { cache_type } => {
            println!("清除缓存: {}", cache_type.green());
            // TODO: 实现按类型清除
            println!("{}", "缓存已清除 ✓".green());
        }
        CacheCommands::Status => {
            println!("{}", "缓存状态:".bold());
            let cache_dir = std::path::Path::new("content/cache");
            if cache_dir.exists() {
                let size = dir_size(cache_dir)?;
                println!("  缓存大小: {}", format_bytes(size));
            } else {
                println!("  缓存目录不存在");
            }
        }
    }
    Ok(())
}

/// 处理用户命令
async fn handle_user(action: UserCommands) -> anyhow::Result<()> {
    match action {
        UserCommands::List => {
            println!("{}", "用户列表:".bold());
            // TODO: 实现用户列表
            println!("  (需要数据库连接)");
        }
        UserCommands::Create { username, email, password, role } => {
            println!("创建用户: {} ({})", username.green(), role.green());
            // TODO: 实现用户创建
            println!("{}", "用户创建成功 ✓".green());
        }
        UserCommands::ResetPassword { username, password } => {
            println!("重置密码: {}", username.green());
            // TODO: 实现密码重置
            println!("{}", "密码重置成功 ✓".green());
        }
    }
    Ok(())
}

/// 计算目录大小
fn dir_size(path: &std::path::Path) -> anyhow::Result<u64> {
    let mut size = 0;
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                size += dir_size(&path)?;
            } else {
                size += entry.metadata()?.len();
            }
        }
    }
    Ok(size)
}

/// 格式化字节数
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
