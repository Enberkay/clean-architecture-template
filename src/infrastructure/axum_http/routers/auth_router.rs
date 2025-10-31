use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;
use anyhow::Result;

use crate::{
    application::services::auth_service::AuthService,
    application::dtos::auth_dto::{LoginRequest, RegisterRequest},
};

#[derive(Clone)]
pub struct AuthRouterState {
    pub auth_service: Arc<AuthService>,
}

pub fn routes(auth_service: Arc<AuthService>) -> Router {
    let state = AuthRouterState { auth_service };

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}

async fn register(
    State(state): State<AuthRouterState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match state.auth_service.register(req).await {
        Ok(res) => Ok(Json(json!({
            "status": "success",
            "data": res
        }))),
        Err(err) => Err((axum::http::StatusCode::BAD_REQUEST, err.to_string())),
    }
}

async fn login(
    State(state): State<AuthRouterState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match state.auth_service.login(req).await {
        Ok(res) => Ok(Json(json!({
            "status": "success",
            "data": res
        }))),
        Err(err) => Err((axum::http::StatusCode::UNAUTHORIZED, err.to_string())),
    }
}
