use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};

use validator::Validate;

use crate::application::{
    dtos::permission_dto::{CreatePermissionRequest, UpdatePermissionRequest, PermissionResponse},
    use_cases::permission_usecase::PermissionUseCase,
};

/// Build router for Permission endpoints
pub fn routes(permission_service: Arc<PermissionUseCase>) -> Router {
    Router::new()
        .route("/", post(create_permission))
        .route("/", get(get_all_permissions))
        .route("/{id}", get(get_permission_by_id))
        .route("/{id}", put(update_permission))
        .route("/{id}", delete(delete_permission))
        .with_state(permission_service)
}

/// POST /permissions
async fn create_permission(
    State(service): State<Arc<PermissionUseCase>>,
    Json(payload): Json<CreatePermissionRequest>,
) -> Result<Json<PermissionResponse>, anyhow::Error> {
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    service.create_permission(payload).await.map(Json)
}

/// GET /permissions
async fn get_all_permissions(
    State(service): State<Arc<PermissionUseCase>>,
) -> Result<Json<Vec<PermissionResponse>>, anyhow::Error> {
    service.get_all_permissions().await.map(Json)
}

/// GET /permissions/{id}
async fn get_permission_by_id(
    State(service): State<Arc<PermissionUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, anyhow::Error> {
    match service.get_permission_by_id(id).await? {
        Some(perm) => Ok(Json(perm)),
        None => Err(anyhow::Error::not_found("Permission not found")),
    }
}

/// PUT /permissions/{id}
async fn update_permission(
    State(service): State<Arc<PermissionUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<PermissionResponse>, anyhow::Error> {
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    // Update และคืนข้อมูลที่อัพเดตแล้วในครั้งเดียว
    Ok(Json(service.update_permission(id, payload).await?))
}

/// DELETE /permissions/{id}
async fn delete_permission(
    State(service): State<Arc<PermissionUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, anyhow::Error> {
    // Delete and return deleted permission data
    Ok(Json(service.delete_permission(id).await?))
}
