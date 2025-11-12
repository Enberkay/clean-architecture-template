use axum::{
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;


use crate::{
    application::{
        use_cases::auth_usecase::AuthUseCase,
        dtos::auth_dto::{LoginRequest, RegisterRequest},
    },

    presentation::http::cookie_utils::{
        set_access_token_cookie,
        set_refresh_token_cookie,
        extract_refresh_token_from_cookie,
        clear_all_auth_cookies,
    },
};

/// Convert anyhow::Error to HTTP response
fn handle_anyhow_error(err: anyhow::Error) -> axum::response::Response {
    let (status, message) = if err.to_string().contains("not found") {
        (StatusCode::NOT_FOUND, err.to_string())
    } else if err.to_string().contains("unauthorized") || err.to_string().contains("Invalid credentials") {
        (StatusCode::UNAUTHORIZED, err.to_string())
    } else if err.to_string().contains("already exists") || err.to_string().contains("conflict") {
        (StatusCode::CONFLICT, err.to_string())
    } else if err.to_string().contains("validation") || err.to_string().contains("invalid") {
        (StatusCode::BAD_REQUEST, err.to_string())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
    };

    let response = json!({
        "success": false,
        "message": message,
        "error": err.to_string()
    });

    (status, Json(response)).into_response()
}

#[derive(Clone)]
pub struct AuthRouterState {
    pub auth_service: Arc<AuthUseCase>,
    pub jwt_repo: Arc<dyn crate::infrastructure::JwtService>,
    pub config: crate::config::config_model::JwtConfig,
}

pub fn routes(
    auth_service: Arc<AuthUseCase>,
    jwt_repo: Arc<dyn crate::infrastructure::JwtService>,
    config: crate::config::config_model::JwtConfig,
) -> Router {
    let state = AuthRouterState { 
        auth_service, 
        jwt_repo, 
        config,
    };

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
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
            tracing::warn!("Invalid register JSON: {}", rejection.body_text());
            let response = json!({
                "success": false,
                "error": format!("Invalid or incomplete JSON body: {}", rejection.body_text()),
                "hint": "Required fields: fname, lname, email, password, age, sex, phone"
            });
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // Validation is now handled in UseCase layer

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

        Err(err) => {
            let error_response = handle_anyhow_error(err);
            error_response
        },
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
            tracing::warn!("Invalid login JSON: {}", rejection.body_text());
            let response = json!({
                "success": false,
                "error": format!("Invalid or incomplete JSON body: {}", rejection.body_text()),
                "hint": "Required fields: email, password"
            });
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // Validation is now handled in UseCase layer
    // --- Call service ---
    match state.auth_service.login(req).await {
        Ok((login_res, access_token, refresh_token)) => {
            let response = (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": login_res
                }))
            ).into_response();

            // Set both AT and RT cookies
            let access_token_age: i64 = (state.config.access_token_expiry_minutes * 60) as i64;
            let refresh_token_age: i64 = (state.config.refresh_token_expiry_days * 24 * 60 * 60) as i64;
            
            let response = set_access_token_cookie(response, &access_token, access_token_age);
            set_refresh_token_cookie(response, &refresh_token, refresh_token_age)
        },

        Err(err) => handle_anyhow_error(err).into_response(),
    }
}

// ===============================
// REFRESH TOKEN HANDLER (สำหรับ F5 และ Interceptor)
// ===============================
async fn refresh_token(
    State(state): State<AuthRouterState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // --- Extract RT from cookie ---
    let refresh_token = match extract_refresh_token_from_cookie(&headers) {
        Some(token) => token,
        None => {
            let response = json!({
                "success": false,
                "error": "Refresh token not found"
            });
            return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }
    };

    // --- Call service ---
    match state.auth_service.refresh_token(&refresh_token).await {
        Ok((refresh_res, new_access_token)) => {
            let response = (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": refresh_res
                }))
            ).into_response();

            // Set new AT cookie (RT ยังเดิม)
            let access_token_age: i64 = (state.config.access_token_expiry_minutes * 60) as i64;
            set_access_token_cookie(response, &new_access_token, access_token_age)
        },

        Err(err) => {
            let error_response = handle_anyhow_error(err);
            clear_all_auth_cookies(error_response).into_response()
        }
    }
}

// ===============================
// LOGOUT HANDLER
// ===============================
async fn logout() -> impl IntoResponse {
    // สำหรับ stateless flow แค่ clear cookies ก็พอ
    let response = (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Logged out successfully"
        }))
    ).into_response();

    clear_all_auth_cookies(response)
}
