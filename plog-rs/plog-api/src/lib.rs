//! Plog CMS API Service
//! 
//! 微内核架构的 API 服务入口

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

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub jwt: Arc<plog_auth::JwtService>,
}

impl FromRef<AppState> for plog_auth::AuthState {
    fn from_ref(state: &AppState) -> Self {
        plog_auth::AuthState {
            // DEVIATION: Arc clone on cold path (request extraction)
            jwt: state.jwt.clone(),
        }
    }
}

#[tracing::instrument(skip_all)]
async fn request_id_middleware(mut request: Request<axum::body::Body>, next: middleware::Next) -> Response {
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

#[tracing::instrument(skip_all)]
pub async fn run(cancel_token: CancellationToken) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "plog_api=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = plog_core::AppConfig::load()
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

    let mut db_opt = ConnectOptions::new(&config.database.url);
    db_opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::Level::DEBUG);
    
    let db = Database::connect(db_opt).await?;
    tracing::info!("Database connected");
    
    plog_migrations::Migrator::up(&db, None).await?;
    tracing::info!("Database migrations completed");

    let jwt = Arc::new(plog_auth::JwtService::new(
        &config.auth.jwt_secret,
        config.auth.jwt_expiration,
    ));

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

    let state = AppState { db, jwt: jwt.clone() };

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .merge(routes::posts::routes())
        .merge(routes::categories::routes())
        .merge(routes::tags::routes())
        .merge(routes::auth::routes())
        .layer(middleware::from_fn(request_id_middleware))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            cancel_token.cancelled().await;
            tracing::info!("Graceful shutdown initiated, draining connections...");
        })
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

async fn root_handler() -> &'static str {
    "Plog CMS API v2 (Microkernel)"
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
