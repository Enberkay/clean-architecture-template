use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use crate::application::{
    dtos::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse},
    services::user_service::UserService,
};

/// Build router for User endpoints
pub fn routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_all_users))
        .route("/{id}", get(get_user_by_id))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
        .with_state(user_service)
}

/// POST /users
async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (axum::http::StatusCode, String)> {
    service
        .create_user(payload)
        .await
        .map(Json)
        .map_err(internal_error)
}

/// GET /users
async fn get_all_users(
    State(service): State<Arc<UserService>>,
) -> Result<Json<Vec<UserResponse>>, (axum::http::StatusCode, String)> {
    service.get_all_users().await.map(Json).map_err(internal_error)
}

/// GET /users/{id}
async fn get_user_by_id(
    State(service): State<Arc<UserService>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, (axum::http::StatusCode, String)> {
    match service.get_user_by_id(id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, "User not found".into())),
        Err(e) => Err(internal_error(e)),
    }
}

/// PUT /users/{id}
async fn update_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match service.update_user(id, payload).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "success"}))),
        Err(e) => Err(internal_error(e)),
    }
}

/// DELETE /users/{id}
async fn delete_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match service.delete_user(id).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "deleted"}))),
        Err(e) => Err(internal_error(e)),
    }
}

/// Standardized error response
fn internal_error<E: std::fmt::Display>(err: E) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
