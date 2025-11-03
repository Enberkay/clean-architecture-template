use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;
use validator::Validate;

use crate::{
    application::{
        application_errors::ApplicationError,
        services::auth_service::AuthService,
        dtos::auth_dto::{LoginRequest, RegisterRequest},
    },
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
) -> Result<Json<serde_json::Value>, ApplicationError> {
    //ตรวจสอบ validation
    if let Err(e) = req.validate() {
        let msg = e
            .field_errors()
            .values()
            .flat_map(|errs| errs.iter().filter_map(|err| err.message.clone()))
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(ApplicationError::bad_request(msg));
    }

    //ดำเนินการ register
    match state.auth_service.register(req).await {
        Ok(res) => Ok(Json(json!({
            "status": "success",
            "data": res
        }))),
        Err(err) => Err(ApplicationError::bad_request(err.to_string())),
    }
}

async fn login(
    State(state): State<AuthRouterState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApplicationError> {
    //ตรวจสอบ validation
    if let Err(e) = req.validate() {
        let msg = e
            .field_errors()
            .values()
            .flat_map(|errs| errs.iter().filter_map(|err| err.message.clone()))
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(ApplicationError::bad_request(msg));
    }

    //ดำเนินการ login
    match state.auth_service.login(req).await {
        Ok(res) => Ok(Json(json!({
            "status": "success",
            "data": res
        }))),
        Err(err) => Err(ApplicationError::unauthorized(err.to_string())),
    }
}
