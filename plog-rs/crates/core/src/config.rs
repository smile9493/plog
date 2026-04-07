//! 配置管理

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// 应用配置
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub auth: AuthSettings,
    pub server: ServerSettings,
    pub cors: CorsSettings,
}

/// 应用设置
#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub env: String,
    pub debug: bool,
}

/// 数据库设置
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

/// 认证设置
#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

/// 服务器设置
#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

/// CORS 设置
#[derive(Debug, Clone, Deserialize)]
pub struct CorsSettings {
    pub allowed_origins: Vec<String>,
}

impl AppConfig {
    /// 加载配置
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // 默认配置
            .set_default("app.name", "Plog CMS")?
            .set_default("app.env", "development")?
            .set_default("app.debug", false)?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 2)?
            .set_default("auth.jwt_expiration", 86400)?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default(
                "cors.allowed_origins",
                vec!["http://localhost:3000", "http://localhost:5173"],
            )?
            // 配置文件
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", env)).required(false))
            // 环境变量
            .add_source(Environment::with_prefix("PLOG").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
