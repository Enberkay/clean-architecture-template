use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
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
) -> Result<Json<RoleResponse>, anyhow::Error> {
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    service.create_role(payload).await.map(Json).map_err(|e| {
        anyhow::Error::internal(format!("Failed to create role: {}", e))
    })
}

/// GET /roles
async fn get_all_roles(
    State(service): State<Arc<RoleUseCase>>,
) -> Result<Json<Vec<RoleResponse>>, anyhow::Error> {
    service.get_all_roles().await.map(Json).map_err(|e| {
        anyhow::Error::internal(format!("Failed to fetch roles: {}", e))
    })
}

/// GET /roles/{id}
async fn get_role_by_id(
    State(service): State<Arc<RoleUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, anyhow::Error> {
    match service.get_role_by_id(id).await.map_err(|e| {
        anyhow::Error::internal(format!("Failed to fetch role: {}", e))
    })? {
        Some(role) => Ok(Json(role)),
        None => Err(anyhow::Error::not_found("Role not found")),
    }
}

/// PUT /roles/{id}
async fn update_role(
    State(service): State<Arc<RoleUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>, anyhow::Error> {
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    // Update และคืนข้อมูลที่อัพเดตแล้วในครั้งเดียว
    Ok(Json(service.update_role(id, payload).await?))
}

/// DELETE /roles/{id}
async fn delete_role(
    State(service): State<Arc<RoleUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, anyhow::Error> {
    // Delete and return deleted role data
    Ok(Json(service.delete_role(id).await?))
}
