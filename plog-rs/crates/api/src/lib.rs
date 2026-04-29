//! Plog CMS API Service
//! 
//! API 服务入口

// CI 强制 Lints - 生产环境安全保护
#![deny(clippy::await_holding_lock)]
#![deny(clippy::await_holding_refcell_ref)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::todo)]
#![deny(clippy::dbg_macro)]
#![deny(unsafe_op_in_unsafe_fn)]

mod routes;

use axum::{routing::get, Router, Json, middleware, http::Request, response::Response};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::extract::FromRef;
use sea_orm::{Database, ConnectOptions};
use plog_migrations::MigratorTrait;
use tokio_util::sync::CancellationToken;

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
            // DEVIATION: Arc clone on cold path (request extraction)
            jwt: state.jwt.clone(),
        }
    }
}

#[tracing::instrument(skip_all)]
async fn request_id_middleware(mut request: Request<axum::body::Body>, next: axum::middleware::Next) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    let header_value: axum::http::HeaderValue = match request_id.parse() {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to parse request ID as header: {}", e);
            return next.run(request).await;
        }
    };
    
    request.headers_mut().insert("x-request-id", header_value.clone());
    
    let span = tracing::info_span!("request", request_id = %request_id);
    let _guard = span.enter();
    
    let start = std::time::Instant::now();
    
    let mut response = next.run(request).await;
    
    let duration = start.elapsed();
    
    response.headers_mut().insert("x-request-id", header_value);
    
    let duration_header = format!("{}ms", duration.as_millis());
    if let Ok(v) = duration_header.parse() {
        response.headers_mut().insert("x-response-time", v);
    }
    
    response
}

/// 启动 API 服务
#[tracing::instrument(skip_all)]
pub async fn run(cancel_token: CancellationToken) -> anyhow::Result<()> {
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
    // Performance Analysis:
    // - max_connections: 根据并发请求量设置，建议 = CPU核数 * 2 + 磁盘数
    // - min_connections: 预热连接，减少冷启动延迟
    // - connect_timeout: 10s 适合局域网数据库
    // - idle_timeout: 300s 平衡资源占用和重连开销
    // - max_lifetime: 1800s 防止长期累积问题 (MySQL wait_timeout)
    // - P3 优化: 生产环境建议启用 sqlx_logging 监控慢查询
    // - 慢查询监控: 设置环境变量 RUST_LOG=sqlx=debug
    let mut db_opt = ConnectOptions::new(&config.database.url);
    db_opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true)  // 启用慢查询监控
        .sqlx_logging_level(tracing::Level::DEBUG);  // DEBUG 级别
    
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
        .map(|o| o.parse().map_err(|e| anyhow::anyhow!("Invalid CORS origin '{}': {}", o, e)))
        .collect::<Result<Vec<_>, _>>()?;
    let cors = CorsLayer::new()
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
            axum::http::Method::PATCH,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
            axum::http::header::ORIGIN,
            axum::http::header::COOKIE,
        ])
        .allow_origin(AllowOrigin::list(cors_origins))
        .allow_credentials(true);

    // 创建速率限制层（每 IP 每分钟 请求）
    // DEVIATION: Rate limiter disabled for current deployment
    // let rate_limiter_config = tower_governor::governor::GovernorConfigBuilder::default()
    //     .per_second(1)
    //     .burst_size(60)
    //     .key_extractor(tower_governor::key_extractor::SmartIpKeyExtractor)
    //     .finish()
    //     .ok_or_else(|| anyhow::anyhow!("Failed to build rate limiter config"))?;

    // 创建应用状态
    let state = AppState { db, jwt: jwt.clone() };

    // 创建路由
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .merge(routes::init::routes())
        .merge(routes::auth::routes())
        .merge(routes::posts::routes())
        .merge(routes::categories::routes())
        .merge(routes::tags::routes())
        .merge(routes::comments::routes())
        .layer(middleware::from_fn(request_id_middleware))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new())
        .layer(cors)
        // .layer(rate_limiter)
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    // 优雅关闭
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            cancel_token.cancelled().await;
            tracing::info!("Graceful shutdown initiated, draining connections...");
        })
        .await?;

    tracing::info!("Server shutdown complete");
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
