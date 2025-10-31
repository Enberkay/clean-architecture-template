use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use crate::application::{
    dtos::role_dto::{CreateRoleRequest, UpdateRoleRequest, RoleResponse},
    services::role_service::RoleService,
};

/// Build router for Role endpoints
pub fn routes(role_service: Arc<RoleService>) -> Router {
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
    State(service): State<Arc<RoleService>>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>, (axum::http::StatusCode, String)> {
    service
        .create_role(payload)
        .await
        .map(Json)
        .map_err(internal_error)
}

/// GET /roles
async fn get_all_roles(
    State(service): State<Arc<RoleService>>,
) -> Result<Json<Vec<RoleResponse>>, (axum::http::StatusCode, String)> {
    service.get_all_roles().await.map(Json).map_err(internal_error)
}

/// GET /roles/{id}
async fn get_role_by_id(
    State(service): State<Arc<RoleService>>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, (axum::http::StatusCode, String)> {
    match service.get_role_by_id(id).await {
        Ok(Some(role)) => Ok(Json(role)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, "Role not found".into())),
        Err(e) => Err(internal_error(e)),
    }
}

/// PUT /roles/{id}
async fn update_role(
    State(service): State<Arc<RoleService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match service.update_role(id, payload).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "updated"}))),
        Err(e) => Err(internal_error(e)),
    }
}

/// DELETE /roles/{id}
async fn delete_role(
    State(service): State<Arc<RoleService>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match service.delete_role(id).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "deleted"}))),
        Err(e) => Err(internal_error(e)),
    }
}

/// Generic error helper
fn internal_error<E: std::fmt::Display>(err: E) -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
