use std::sync::Arc;

use clean_architecture_template::{
    infrastructure::config_loader,
    adapters::postgres::postgres_connector,
};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load .env
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Warning: couldn't load .env file: {}", e);
    }

    // Initialize tracing (with env filter fallback)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .compact()
        .init();

    // Load configuration
    let app_config = match config_loader::load() {
        Ok(cfg) => {
            info!("Configuration loaded successfully for environment: {:?}", cfg.environment);
            cfg
        }
        Err(e) => {
            error!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };

    // Connect to PostgreSQL
    let pg_pool = match postgres_connector::establish_connection(&app_config.database.url).await {
        Ok(pool) => {
            info!("PostgreSQL connection pool established");
            pool
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL: {:?}", e);
            std::process::exit(1);
        }
    };

    // Start HTTP server
    info!("Starting Bookstore backend on port {}...", app_config.server.port);
}
