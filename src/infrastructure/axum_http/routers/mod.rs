// src/infrastructure/axum_http/routers/mod.rs
use std::sync::Arc;
use axum::{routing::get, Router};
use crate::infrastructure::postgres::postgres_connector::PgPoolSquad;

pub fn routes(_db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Bookstore API Root"
}
