use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use validator::Validate;

use crate::application::{

    dtos::branch_dto::{CreateBranchRequest, UpdateBranchRequest, BranchResponse},
    use_cases::branch_usecase::BranchUseCase,
};

/// Build router for Branch endpoints
pub fn routes(branch_service: Arc<BranchUseCase>) -> Router {
    Router::new()
        .route("/", post(create_branch))
        .route("/", get(get_all_branches))
        .route("/{id}", get(get_branch_by_id))
        .route("/{id}", put(update_branch))
        .route("/{id}", delete(delete_branch))
        .with_state(branch_service)
}

/// POST /branches
async fn create_branch(
    State(service): State<Arc<BranchUseCase>>,
    Json(payload): Json<CreateBranchRequest>,
) -> Result<Json<BranchResponse>, anyhow::Error> {
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    Ok(Json(service.create_branch(payload).await?))
}

/// GET /branches
async fn get_all_branches(
    State(service): State<Arc<BranchUseCase>>,
) -> Result<Json<Vec<BranchResponse>>, anyhow::Error> {
    Ok(Json(service.get_all_branches().await?))
}

/// GET /branches/{id}
async fn get_branch_by_id(
    State(service): State<Arc<BranchUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<BranchResponse>, anyhow::Error> {
    match service.get_branch_by_id(id).await? {
        Some(branch) => Ok(Json(branch)),
        None => Err(anyhow::Error::not_found("Branch not found")),
    }
}

/// PUT /branches/{id}
async fn update_branch(
    State(service): State<Arc<BranchUseCase>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBranchRequest>,
) -> Result<Json<BranchResponse>, anyhow::Error> {
    payload
        .validate()
        .map_err(|e| anyhow::Error::bad_request(e.to_string()))?;

    Ok(Json(service.update_branch(id, payload).await?))
}

/// DELETE /branches/{id}
async fn delete_branch(
    State(service): State<Arc<BranchUseCase>>,
    Path(id): Path<i32>,
) -> Result<Json<BranchResponse>, anyhow::Error> {
    Ok(Json(service.delete_branch(id).await?))
}
