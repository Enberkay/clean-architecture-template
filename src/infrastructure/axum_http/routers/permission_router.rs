use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use crate::application::{
    dtos::permission_dto::{CreatePermissionRequest, UpdatePermissionRequest, PermissionResponse},
    services::permission_service::PermissionService,
};

/// Build router for Permission endpoints
pub fn routes(permission_service: Arc<PermissionService>) -> Router {
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
    State(service): State<Arc<PermissionService>>,
    Json(payload): Json<CreatePermissionRequest>,
) -> Result<Json<PermissionResponse>, (axum::http::StatusCode, String)> {
    service
        .create_permission(payload)
        .await
        .map(Json)
        .map_err(internal_error)
}

/// GET /permissions
async fn get_all_permissions(
    State(service): State<Arc<PermissionService>>,
) -> Result<Json<Vec<PermissionResponse>>, (axum::http::StatusCode, String)> {
    service
        .get_all_permissions()
        .await
        .map(Json)
        .map_err(internal_error)
}

/// GET /permissions/{id}
async fn get_permission_by_id(
    State(service): State<Arc<PermissionService>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, (axum::http::StatusCode, String)> {
    match service.get_permission_by_id(id).await {
        Ok(Some(perm)) => Ok(Json(perm)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, "Permission not found".into())),
        Err(e) => Err(internal_error(e)),
    }
}

/// PUT /permissions/{id}
async fn update_permission(
    State(service): State<Arc<PermissionService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match service.update_permission(id, payload).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "updated"}))),
        Err(e) => Err(internal_error(e)),
    }
}

/// DELETE /permissions/{id}
async fn delete_permission(
    State(service): State<Arc<PermissionService>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match service.delete_permission(id).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "deleted"}))),
        Err(e) => Err(internal_error(e)),
    }
}

/// Helper for consistent error handling
fn internal_error<E: std::fmt::Display>(err: E) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
