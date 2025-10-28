use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::AppConfig,
    infrastructure::{
        axum_http::{
            default_router,
            routers,
        },
        postgres::postgres_connector::PgPoolSquad,
    },
};

/// Start the Axum HTTP server with middleware stack
pub async fn start_server(config: Arc<AppConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    // === Base router ===
    let base_router = Router::new()
        .route("/health", get(default_router::health_check))
        .fallback(default_router::not_found)
        .nest("/api", routers::routes(Arc::clone(&db_pool)));

    // === Global middleware stack ===
    let app = base_router
        // Limit request body size
        .route_layer(RequestBodyLimitLayer::new(
            config.server.body_limit.try_into().unwrap_or(10_485_760), // 10MB fallback
        ))
        // Apply timeout globally
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout_seconds as u64,
        )))
        // Add permissive CORS (development default)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers(Any),
        )
        // Structured request tracing (keep last)
        .layer(TraceLayer::new_for_http());

    // === Start server ===
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    info!(
        "Bookstore backend running on http://0.0.0.0:{} ({:?} mode)",
        config.server.port, config.environment
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Graceful shutdown handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    // Keeps select alive on non-Unix (no SIGTERM)
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C signal — shutting down gracefully"),
        _ = terminate => info!("Received terminate signal"),
    }
}
