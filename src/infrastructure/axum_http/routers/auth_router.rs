use axum::{
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;
use validator::Validate;
use tracing::warn;

use crate::{
    application::{
        services::auth_service::AuthService,
        dtos::auth_dto::{LoginRequest, RegisterRequest},
    },
    infrastructure::axum_http::cookie_utils::set_refresh_token_cookie,
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

// ===============================
// REGISTER HANDLER
// ===============================
async fn register(
    State(state): State<AuthRouterState>,
    payload: Result<Json<RegisterRequest>, JsonRejection>,
) -> impl IntoResponse {
    // --- Handle invalid JSON ---
    let req = match payload {
        Ok(Json(req)) => req,
        Err(rejection) => {
            warn!("Invalid register JSON: {}", rejection.body_text());
            let response = json!({
                "success": false,
                "error": format!("Invalid or incomplete JSON body: {}", rejection.body_text()),
                "hint": "Required fields: fname, lname, email, password, age, sex, phone"
            });
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // --- Validate DTO ---
    if let Err(e) = req.validate() {
        let msg = e
            .field_errors()
            .values()
            .flat_map(|errs| errs.iter().filter_map(|err| err.message.clone()))
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let response = json!({ "success": false, "error": msg });
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    // --- Call service ---
    match state.auth_service.register(req).await {
        Ok(res) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "data": res
            })),
        )
            .into_response(),

        Err(err) => err.into_response(), //ApplicationError → JSON response
    }
}

// ===============================
// LOGIN HANDLER
// ===============================
async fn login(
    State(state): State<AuthRouterState>,
    payload: Result<Json<LoginRequest>, JsonRejection>,
) -> impl IntoResponse {
    // --- Handle invalid JSON ---
    let req = match payload {
        Ok(Json(req)) => req,
        Err(rejection) => {
            warn!("Invalid login JSON: {}", rejection.body_text());
            let response = json!({
                "success": false,
                "error": format!("Invalid or incomplete JSON body: {}", rejection.body_text()),
                "hint": "Required fields: email, password"
            });
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // --- Validate DTO ---
    if let Err(e) = req.validate() {
        let msg = e
            .field_errors()
            .values()
            .flat_map(|errs| errs.iter().filter_map(|err| err.message.clone()))
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let response = json!({ "success": false, "error": msg });
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    // --- Call service ---
    match state.auth_service.login(req).await {
        Ok((login_res, refresh_token)) => {
            let response = (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": login_res
                }))
            ).into_response();

            set_refresh_token_cookie(response, &refresh_token)
        },

        Err(err) => err.into_response(),
    }
}
