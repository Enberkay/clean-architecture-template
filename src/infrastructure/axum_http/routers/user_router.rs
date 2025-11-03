use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde_json::json;
use validator::Validate;

use crate::application::{
    application_errors::ApplicationError,
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
) -> Result<Json<UserResponse>, ApplicationError> {
    //Validate request payload
    payload
        .validate()
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

    service.create_user(payload).await.map(Json)
}

/// GET /users
async fn get_all_users(
    State(service): State<Arc<UserService>>,
) -> Result<Json<Vec<UserResponse>>, ApplicationError> {
    service.get_all_users().await.map(Json)
}

/// GET /users/{id}
async fn get_user_by_id(
    State(service): State<Arc<UserService>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, ApplicationError> {
    match service.get_user_by_id(id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(ApplicationError::not_found("User not found")),
    }
}

/// PUT /users/{id}
async fn update_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<serde_json::Value>, ApplicationError> {
    //Validate request payload
    payload
        .validate()
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

    service.update_user(id, payload).await?;
    Ok(Json(json!({ "status": "success" })))
}

/// DELETE /users/{id}
async fn delete_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, ApplicationError> {
    service.delete_user(id).await?;
    Ok(Json(json!({ "status": "deleted" })))
}
