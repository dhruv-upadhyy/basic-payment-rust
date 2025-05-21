mod base;
mod db;
mod api;
use std::time::Duration;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;
use axum::Extension;
use api::middleware::rate_limit::RateLimiter;

#[tokio::main]
async fn main() {
    let config = base::config::AppConfig::from_env()
        .expect("Failed to load configuration");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = config.pg_pool();

    // 100 requests per minute per IP
    let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));

    let app = api::routes::create_router(pool)
        .layer(Extension(rate_limiter))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
