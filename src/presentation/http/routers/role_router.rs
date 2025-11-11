use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router, http::StatusCode,
};

use validator::Validate;

use crate::application::{
    dtos::role_dto::{CreateRoleRequest, UpdateRoleRequest, RoleResponse},
    use_cases::role_usecase::RoleUseCase,
};

/// Build router for Role endpoints
pub fn routes(role_service: Arc<RoleUseCase>) -> Router {
    Router::new()
        .route("/", post(create_role))
        .route("/", get(get_all_roles))
        .route("/{id}", get(get_role_by_id))
        .route("/{id}", put(update_role))
        .route("/{id}", delete(delete_role))
        .with_state(role_service)
}

/// POST /roles
async fn create_role(
    State(service): State<Arc<RoleUseCase>>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, anyhow::anyhow!("Validation error: {}", e).to_string()))?;

    match service.create_role(payload).await {
        Ok(role) => Ok(Json(role)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, anyhow::anyhow!("Failed to create role: {}", e).to_string())),
    }
}

/// GET /roles
async fn get_all_roles(
    State(service): State<Arc<RoleUseCase>>,
) -> Result<Json<Vec<RoleResponse>>, (StatusCode, String)> {
    match service.get_all_roles().await {
        Ok(roles) => Ok(Json(roles)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, anyhow::anyhow!("Failed to fetch roles: {}", e).to_string())),
    }
}

/// GET /roles/{id}
async fn get_role_by_id(
    State(service): State<Arc<RoleUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    match service.get_role_by_id(id).await {
        Ok(result) => {
            match result {
                Some(role) => Ok(Json(role)),
                None => Err((StatusCode::NOT_FOUND, anyhow::anyhow!("Role not found").to_string())),
            }
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, anyhow::anyhow!("Failed to fetch role: {}", e).to_string())),
    }
}

/// PUT /roles/{id}
async fn update_role(
    State(service): State<Arc<RoleUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, anyhow::anyhow!("Validation error: {}", e).to_string()))?;

    // Update และคืนข้อมูลที่อัพเดตแล้วในครั้งเดียว
    match service.update_role(id, payload).await {
        Ok(role) => Ok(Json(role)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// DELETE /roles/{id}
async fn delete_role(
    State(service): State<Arc<RoleUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, (StatusCode, String)> {
    // Delete and return deleted role data
    match service.delete_role(id).await {
        Ok(role) => Ok(Json(role)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
