use std::sync::Arc;

use bookstore_backend::{
    config::config_loader,
    infrastructure::postgres::postgres_connector,
    presentation::http::http_serve::start_server,
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Warning: Couldn't load .env file: {}", e);
    }

    // Initialize structured tracing logger
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

    // Load environment configuration
    let app_config = match config_loader::load() {
        Ok(config) => {
            info!("Environment variables loaded successfully");
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Connect to Postgres (async)
    let pg_pool = match postgres_connector::establish_connection(&app_config.database.url).await {
        Ok(pool) => {
            info!("PostgreSQL connection pool established");
            pool
        }
        Err(e) => {
            error!("❌ Failed to connect to PostgreSQL: {}", e);
            std::process::exit(1);
        }
    };

    // Start the HTTP server
    info!(
        "Starting Bookstore backend on port {}...",
        app_config.server.port
    );

    if let Err(e) = start_server(Arc::new(app_config), Arc::new(pg_pool)).await {
        error!("Server encountered an error: {}", e);
    }
}
