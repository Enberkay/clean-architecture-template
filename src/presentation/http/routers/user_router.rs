use std::sync::Arc;
use axum::{
    Json, Router, extract::{Path, State}, response::IntoResponse, routing::{delete, get, patch, post, put},

};

use validator::Validate;
use crate::presentation::http::middlewares::jwt_auth::JwtAuthLayer;

use crate::application::{

    dtos::user_dto::{CreateUserRequest, UpdatePasswordRequest, UpdateUserRequest, UserResponse},
    use_cases::user_usecase::UserUseCase,
};

/// Build router for User endpoints
pub fn routes(user_service: Arc<UserUseCase>, jwt_secret: String) -> Router {
    let jwt_middleware = JwtAuthLayer::new(jwt_secret);
    
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/", post(create_user))
        .with_state(user_service.clone());
    
    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/", get(get_all_users))
        .route("/{id}", get(get_user_by_id))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}/deactivate", patch(deactivate_user))
        .route("/{id}/activate", patch(activate_user))
        .route("/{id}/password", patch(update_password))
        .with_state(user_service)
        .layer(jwt_middleware);
    
    // Combine both routers
    Router::new().merge(public_routes).merge(protected_routes)
}

/// POST /users
async fn create_user(
    State(service): State<Arc<UserUseCase>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, anyhow::Error> {
    //Validate request payload
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    service.create_user(payload).await.map(Json)
}

/// GET /users
async fn get_all_users(
    State(service): State<Arc<UserUseCase>>,
) -> Result<Json<Vec<UserResponse>>, anyhow::Error> {
    service.get_all_users().await.map(Json)
}

/// GET /users/{id}
async fn get_user_by_id(
    State(service): State<Arc<UserUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, anyhow::Error> {
    match service.get_user_by_id(id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(anyhow::Error::not_found("User not found")),
    }
}

/// PUT /users/{id}
async fn update_user(
    State(service): State<Arc<UserUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, anyhow::Error> {
    //Validate request payload
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    // Update and return updated user data
    Ok(Json(service.update_user(id, payload).await?))
}

/// DELETE /users/{id}
async fn delete_user(
    State(service): State<Arc<UserUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, anyhow::Error> {
    // Delete and return deleted user data
    Ok(Json(service.delete_user(id).await?))
}

async fn deactivate_user(
    Path(id): Path<i32>,
    State(user_service): State<Arc<UserUseCase>>,
) -> Result<impl IntoResponse, anyhow::Error> {
    let user = user_service.deactivate_user(id).await?;
    Ok(Json(user))
}

async fn activate_user(
    Path(id): Path<i32>,
    State(service): State<Arc<UserUseCase>>,
) -> Result<Json<serde_json::Value>, anyhow::Error> {
    let user = service.activate_user(id).await?;
    Ok(Json(serde_json::json!({ "status": "activated", "user": user })))
}

async fn update_password(
    Path(id): Path<i32>,
    State(service): State<Arc<UserUseCase>>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<Json<serde_json::Value>, anyhow::Error> {
    let user = service.update_password(id, payload).await?;
    Ok(Json(serde_json::json!({
        "status": "password_updated",
        "user": user
    })))
}
