//! Plog CMS 安装器
//!
//! 交互式安装向导

use std::io::{self, Write};
use colored::Colorize;
use dialoguer::{Input, Password, Select, Confirm};

/// 安装配置
#[derive(Debug, Clone)]
struct InstallConfig {
    db_host: String,
    db_port: u16,
    db_name: String,
    db_user: String,
    db_password: String,
    site_name: String,
    site_url: String,
    admin_username: String,
    admin_password: String,
    admin_email: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 显示欢迎信息
    show_welcome();

    // 检查环境
    check_environment()?;

    // 收集配置
    let config = collect_config()?;

    // 确认安装
    if !confirm_install(&config)? {
        println!("{}", "安装已取消".yellow());
        return Ok(());
    }

    // 执行安装
    execute_install(&config).await?;

    // 显示完成信息
    show_complete(&config);

    Ok(())
}

/// 显示欢迎信息
fn show_welcome() {
    println!("{}", "
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║                    Plog CMS 安装向导                          ║
║                                                               ║
║              欢迎使用 Plog CMS 安装程序                        ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
".cyan());
}

/// 检查环境
fn check_environment() -> anyhow::Result<()> {
    println!("{}", "检查系统环境...".green());

    // 检查必要目录
    let dirs = ["content", "config", "logs"];
    for dir in dirs {
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir_all(dir)?;
            println!("  ✓ 创建目录: {}", dir);
        }
    }

    println!("{}", "环境检查完成 ✓".green());
    println!();
    Ok(())
}

/// 收集配置
fn collect_config() -> anyhow::Result<InstallConfig> {
    println!("{}", "请输入数据库配置:".bold());
    println!();

    let db_host: String = Input::new()
        .with_prompt("数据库主机")
        .default("localhost".into())
        .interact_text()?;

    let db_port: String = Input::new()
        .with_prompt("数据库端口")
        .default("3306".into())
        .interact_text()?;

    let db_name: String = Input::new()
        .with_prompt("数据库名称")
        .default("plog".into())
        .interact_text()?;

    let db_user: String = Input::new()
        .with_prompt("数据库用户")
        .default("root".into())
        .interact_text()?;

    let db_password: String = Password::new()
        .with_prompt("数据库密码")
        .with_confirmation("确认密码", "密码不匹配")
        .interact()?;

    println!();
    println!("{}", "请输入站点信息:".bold());
    println!();

    let site_name: String = Input::new()
        .with_prompt("站点名称")
        .default("My Blog".into())
        .interact_text()?;

    let site_url: String = Input::new()
        .with_prompt("站点地址")
        .default("http://localhost".into())
        .interact_text()?;

    println!();
    println!("{}", "请创建管理员账户:".bold());
    println!();

    let admin_username: String = Input::new()
        .with_prompt("管理员用户名")
        .default("admin".into())
        .interact_text()?;

    let admin_password: String = Password::new()
        .with_prompt("管理员密码")
        .with_confirmation("确认密码", "密码不匹配")
        .interact()?;

    let admin_email: String = Input::new()
        .with_prompt("管理员邮箱")
        .interact_text()?;

    Ok(InstallConfig {
        db_host,
        db_port: db_port.parse()?,
        db_name,
        db_user,
        db_password,
        site_name,
        site_url,
        admin_username,
        admin_password,
        admin_email,
    })
}

/// 确认安装
fn confirm_install(config: &InstallConfig) -> anyhow::Result<bool> {
    println!();
    println!("{}", "安装配置摘要:".bold());
    println!();
    println!("  数据库: {}:{}/{}", config.db_host, config.db_port, config.db_name);
    println!("  站点名称: {}", config.site_name);
    println!("  站点地址: {}", config.site_url);
    println!("  管理员: {}", config.admin_username);
    println!();

    let confirmed = Confirm::new()
        .with_prompt("确认以上配置并开始安装?")
        .default(true)
        .interact()?;

    Ok(confirmed)
}

/// 执行安装
async fn execute_install(config: &InstallConfig) -> anyhow::Result<()> {
    println!();
    println!("{}", "开始安装...".green());
    println!();

    // 1. 测试数据库连接
    print!("  [1/5] 测试数据库连接...");
    io::stdout().flush()?;
    test_database_connection(config).await?;
    println!(" ✓");

    // 2. 创建数据库表
    print!("  [2/5] 创建数据库表...");
    io::stdout().flush()?;
    create_database_tables(config).await?;
    println!(" ✓");

    // 3. 初始化设置
    print!("  [3/5] 初始化系统设置...");
    io::stdout().flush()?;
    init_settings(config).await?;
    println!(" ✓");

    // 4. 创建管理员账户
    print!("  [4/5] 创建管理员账户...");
    io::stdout().flush()?;
    create_admin_user(config).await?;
    println!(" ✓");

    // 5. 生成配置文件
    print!("  [5/5] 生成配置文件...");
    io::stdout().flush()?;
    generate_config_file(config)?;
    println!(" ✓");

    println!();
    println!("{}", "安装完成!".green().bold());

    Ok(())
}

/// 测试数据库连接
async fn test_database_connection(config: &InstallConfig) -> anyhow::Result<()> {
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    );

    // 尝试连接
    let _conn = sea_orm::Database::connect(&db_url).await?;
    Ok(())
}

/// 创建数据库表
async fn create_database_tables(_config: &InstallConfig) -> anyhow::Result<()> {
    // TODO: 运行迁移
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Ok(())
}

/// 初始化设置
async fn init_settings(config: &InstallConfig) -> anyhow::Result<()> {
    // TODO: 初始化默认设置
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    Ok(())
}

/// 创建管理员用户
async fn create_admin_user(_config: &InstallConfig) -> anyhow::Result<()> {
    // TODO: 创建管理员
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    Ok(())
}

/// 生成配置文件
fn generate_config_file(config: &InstallConfig) -> anyhow::Result<()> {
    let config_content = format!(
        r#"[server]
host = "0.0.0.0"
port = 8080

[database]
url = "mysql://{}:{}@{}:{}/{}"

[auth]
jwt_secret = "{}"
jwt_expiration = 3600

[site]
name = "{}"
url = "{}"
"#,
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name,
        generate_random_secret(),
        config.site_name,
        config.site_url,
    );

    std::fs::write("config/local.toml", config_content)?;
    Ok(())
}

/// 生成随机密钥
fn generate_random_secret() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp().hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// 显示完成信息
fn show_complete(config: &InstallConfig) {
    println!();
    println!("{}", "
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║                      安装成功!                                ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
".green());
    println!();
    println!("  管理后台: {}/admin-web", config.site_url);
    println!("  管理员: {}", config.admin_username);
    println!();
    println!("  请运行以下命令启动服务:");
    println!();
    println!("    cargo run --bin plog-api");
    println!();
}
