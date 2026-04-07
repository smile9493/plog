//! Plog CMS API Service
//! 
//! API 服务入口

mod routes;

use axum::{routing::get, Router, Json, middleware, http::Request, response::Response};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{CorsLayer, Any, AllowOrigin};
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_governor::{GovernorLayer, key_extractor::SmartIpKeyExtractor, governor::GovernorConfigBuilder};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::extract::FromRef;
use sea_orm::{Database, ConnectOptions};
use plog_migrations::MigratorTrait;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub jwt: Arc<plog_auth::JwtService>,
}

/// 从 AppState 提取 AuthState
impl FromRef<AppState> for plog_auth::AuthState {
    fn from_ref(state: &AppState) -> Self {
        plog_auth::AuthState {
            jwt: state.jwt.clone(),
        }
    }
}

/// 请求 ID 中间件
async fn request_id_middleware(mut request: Request<axum::body::Body>, next: axum::middleware::Next) -> Response {
    // 生成请求 ID
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // 将请求 ID 添加到请求头
    request.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    // 将请求 ID 添加到 tracing span
    let span = tracing::info_span!("request", request_id = %request_id);
    let _guard = span.enter();
    
    // 记录请求开始时间
    let start = std::time::Instant::now();
    
    // 继续处理请求
    let mut response = next.run(request).await;
    
    // 计算请求耗时
    let duration = start.elapsed();
    
    // 将请求 ID 添加到响应头
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    // 添加响应时间头
    response.headers_mut().insert(
        "x-response-time",
        format!("{}ms", duration.as_millis()).parse().unwrap(),
    );
    
    response
}

/// 启动 API 服务
pub async fn run() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "plog_api=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = plog_core::config::AppConfig::load()
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

    // 配置数据库连接池
    let mut db_opt = ConnectOptions::new(&config.database.url);
    db_opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(false);
    
    // 连接数据库
    let db = Database::connect(db_opt).await?;
    
    tracing::info!(
        "Database connected: max_connections={}, min_connections={}",
        config.database.max_connections,
        config.database.min_connections
    );

    // 运行数据库迁移
    tracing::info!("Running database migrations...");
    plog_migrations::Migrator::up(&db, None).await?;
    tracing::info!("Database migrations completed");

    // 创建 JWT 服务
    let jwt = Arc::new(plog_auth::JwtService::new(
        &config.auth.jwt_secret,
        config.auth.jwt_expiration,
    ));

    // 创建 CORS 层（白名单模式）
    let cors_origins: Vec<_> = config.cors.allowed_origins.iter()
        .map(|o| o.parse().expect("Invalid CORS origin"))
        .collect();
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(AllowOrigin::list(cors_origins));

    // 创建速率限制层（每 IP 每分钟 60 请求）
    let rate_limiter = GovernorLayer {
        config: std::sync::Arc::new(
            GovernorConfigBuilder::default()
                .per_second(1)
                .burst_size(60)
                .key_extractor(SmartIpKeyExtractor)
                .finish()
                .expect("Failed to build rate limiter config"),
        ),
    };

    // 创建应用状态
    let state = AppState { db, jwt: jwt.clone() };

    // 创建路由
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .merge(routes::auth::routes())
        .merge(routes::posts::routes())
        .merge(routes::categories::routes())
        .merge(routes::tags::routes())
        .merge(routes::comments::routes())
        .layer(middleware::from_fn(request_id_middleware))
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(rate_limiter)
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 根路由处理
async fn root_handler() -> &'static str {
    "Plog CMS API v2"
}

/// 健康检查处理
async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
