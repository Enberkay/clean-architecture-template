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
        argon2_hashing::Argon2PasswordHasher,
        axum_http::{default_router, routers},
        jwt_authentication::JwtTokenService,
        postgres::{
            postgres_connector::PgPoolSquad,
            repositories::{
                branch_repository::PostgresBranchRepository,
                permission_repository::PostgresPermissionRepository,
                role_permission_repository::PostgresRolePermissionRepository,
                role_repository::PostgresRoleRepository,
                user_repository::PostgresUserRepository,
            },
        },
        redis::{redis_connector::RedisPool, redis_token_repository::RedisTokenRepository},
    },
    application::services::{
        auth_service::AuthService,
        branch_service::BranchService,
        permission_service::PermissionService,
        role_service::RoleService,
        user_service::UserService,
    },
};

/// Starts the Axum HTTP server with all routers configured.
pub async fn start_server(config: Arc<AppConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    // --- Redis setup ---
    let redis_pool = Arc::new(RedisPool::new(&config.redis).await?);
    let redis_token_repo = Arc::new(RedisTokenRepository::new(
        redis_pool.as_ref().clone(),
        config.redis.refresh_token_expiry_days,
    ));

    // --- Base repositories ---
    let user_repo = Arc::new(PostgresUserRepository::new(db_pool.as_ref().clone()));
    let role_repo = Arc::new(PostgresRoleRepository::new(db_pool.as_ref().clone()));
    let perm_repo = Arc::new(PostgresPermissionRepository::new(db_pool.as_ref().clone()));
    let role_perm_repo = Arc::new(PostgresRolePermissionRepository::new(db_pool.as_ref().clone()));
    let branch_repo = Arc::new(PostgresBranchRepository::new(db_pool.as_ref().clone()));

    // --- Security components ---
    let password_repo = Arc::new(Argon2PasswordHasher::new(
        config.security.argon2_memory_cost,
        config.security.argon2_time_cost,
        config.security.argon2_parallelism,
    ));
    let jwt_repo = Arc::new(JwtTokenService::new(
        &config.users_secret.secret,
        &config.users_secret.refresh_secret,
        config.jwt.access_token_expiry_minutes,
    ));

    // --- Services ---
    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        password_repo,
        jwt_repo.clone(),
        redis_token_repo,
    ));
    let user_service = Arc::new(UserService::new(user_repo.clone(), role_repo.clone()));
    let role_service = Arc::new(RoleService::new(
        role_repo,
        perm_repo.clone(),
        role_perm_repo,
    ));
    let permission_service = Arc::new(PermissionService::new(perm_repo));
    let branch_service = Arc::new(BranchService::new(branch_repo));

    // --- Health router ---
    let health_router = Router::new().route("/health", get(default_router::health_check));

    // --- Application router ---
    let app = Router::new()
        .merge(health_router)
        .fallback(default_router::not_found)
        .nest("/auth", routers::auth_router::routes(auth_service, jwt_repo.clone()))
        .nest("/users", routers::user_router::routes(user_service))
        .nest("/roles", routers::role_router::routes(role_service))
        .nest("/permissions", routers::permission_router::routes(permission_service))
        .nest("/branches", routers::branch_router::routes(branch_service)) // ✅ Added branch routes
        // --- Global middlewares ---
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout_seconds.into(),
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024)
                .try_into()
                .unwrap_or(10 * 1024 * 1024),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    // --- Bind and serve ---
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    info!(
        "Bookstore backend running at http://0.0.0.0:{} in {:?} mode",
        config.server.port, config.environment
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Graceful shutdown handler (Ctrl+C or SIGTERM)
async fn shutdown_signal() {
    #[cfg(unix)]
    use tokio::signal::unix::{signal, SignalKind};

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sigterm) = signal(SignalKind::terminate()) {
            sigterm.recv().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C — shutting down gracefully"),
        _ = terminate => info!("Received SIGTERM — shutting down gracefully"),
    }
}
