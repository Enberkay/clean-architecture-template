use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router, http::StatusCode,
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
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, anyhow::anyhow!("Validation error: {}", e).to_string()))?;
    
    match service.create_permission(payload).await {
        Ok(permission) => Ok(Json(permission)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// GET /permissions
async fn get_all_permissions(
    State(service): State<Arc<PermissionUseCase>>,
) -> Result<Json<Vec<PermissionResponse>>, (StatusCode, String)> {
    match service.get_all_permissions().await {
        Ok(permissions) => Ok(Json(permissions)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// GET /permissions/{id}
async fn get_permission_by_id(
    State(service): State<Arc<PermissionUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    match service.get_permission_by_id(id).await {
        Ok(permission_opt) => {
            match permission_opt {
                Some(perm) => Ok(Json(perm)),
                None => Err((StatusCode::NOT_FOUND, "Permission not found".to_string())),
            }
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, anyhow::anyhow!("Failed to fetch permission: {}", e).to_string())),
    }
}

/// PUT /permissions/{id}
async fn update_permission(
    State(service): State<Arc<PermissionUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    payload
        .validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, anyhow::anyhow!("Validation error: {}", e).to_string()))?;

    // Update และคืนข้อมูลที่อัพเดตแล้วในครั้งเดียว
    match service.update_permission(id, payload).await {
        Ok(permission) => Ok(Json(permission)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// DELETE /permissions/{id}
async fn delete_permission(
    State(service): State<Arc<PermissionUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, (StatusCode, String)> {
    // Delete and return deleted permission data
    match service.delete_permission(id).await {
        Ok(permission) => Ok(Json(permission)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
