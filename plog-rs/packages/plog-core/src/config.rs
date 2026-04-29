//! Plog CMS Configuration
//! 
//! 应用程序配置加载和管理

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

fn default_min_connections() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl AppConfig {
    /// 从默认路径加载配置
    pub fn load() -> Result<Self, config::ConfigError> {
        Self::load_from_path("config/settings.toml")
    }

    /// 从指定路径加载配置
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .build()?;
        
        settings.try_deserialize()
    }

    /// 从环境变量加载配置
    pub fn load_with_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(Path::new("config/settings.toml")))
            .add_source(config::Environment::with_prefix("PLOG").separator("__"))
            .build()?;
        
        settings.try_deserialize()
    }
}
