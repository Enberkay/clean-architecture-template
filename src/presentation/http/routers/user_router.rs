use std::sync::Arc;
use axum::{
    Json, Router, extract::{Path, State}, routing::{delete, get, patch, post, put}, http::StatusCode,
};

use validator::Validate;
use crate::application::{
    dtos::user_dto::{CreateUserRequest, UpdatePasswordRequest, UpdateUserRequest, UserResponse},
    use_cases::user_usecase::UserUseCase,
};

/// Build router for User endpoints
pub fn routes(user_service: Arc<UserUseCase>, _secret: String) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_all_users))
        .route("/{id}", get(get_user_by_id))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}/deactivate", patch(deactivate_user))
        .route("/{id}/activate", patch(activate_user))
        .route("/{id}/password", patch(update_password))
        .with_state(user_service)
}

/// POST /users
async fn create_user(
    State(service): State<Arc<UserUseCase>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    //Validate request payload
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, anyhow::anyhow!("Validation error: {}", e).to_string()))?;
    
    match service.create_user(payload).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// GET /users
async fn get_all_users(
    State(service): State<Arc<UserUseCase>>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    match service.get_all_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// GET /users/{id}
async fn get_user_by_id(
    State(service): State<Arc<UserUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match service.get_user_by_id(id).await {
        Ok(user_opt) => {
            match user_opt {
                Some(user) => Ok(Json(user)),
                None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
            }
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// PUT /users/{id}
async fn update_user(
    State(service): State<Arc<UserUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    //Validate request payload
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, anyhow::anyhow!("Validation error: {}", e).to_string()))?;
 
    // Update and return updated user data
    match service.update_user(id, payload).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// DELETE /users/{id}
async fn delete_user(
    State(service): State<Arc<UserUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    // Delete and return deleted user data
    match service.delete_user(id).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn deactivate_user(
    Path(id): Path<i32>,
    State(user_service): State<Arc<UserUseCase>>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match user_service.deactivate_user(id).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn activate_user(
    Path(id): Path<i32>,
    State(service): State<Arc<UserUseCase>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match service.activate_user(id).await {
        Ok(user) => Ok(Json(serde_json::json!({ "status": "activated", "user": user }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn update_password(
    Path(id): Path<i32>,
    State(service): State<Arc<UserUseCase>>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match service.update_password(id, payload).await {
        Ok(user) => Ok(Json(serde_json::json!({
            "status": "password_updated",
            "user": user
        }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}