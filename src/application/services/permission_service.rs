use std::sync::Arc;
use crate::application::application_errors::{ApplicationError, ApplicationResult};
use crate::application::dtos::permission_dto::{
    CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest,
};
use crate::domain::{
    entities::permission::PermissionEntity,
    repositories::permission_repository::PermissionRepository,
};

pub struct PermissionService {
    repo: Arc<dyn PermissionRepository>,
}

impl PermissionService {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_permission(&self, req: CreatePermissionRequest) -> ApplicationResult<PermissionResponse> {
        let permission = PermissionEntity::new(req.name, req.description)
            .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        self.repo.save(&permission).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to save permission: {}", e))
        })?;

        Ok(PermissionResponse::from(permission))
    }

    pub async fn get_all_permissions(&self) -> ApplicationResult<Vec<PermissionResponse>> {
        let permissions = self.repo.find_all().await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch permissions: {}", e))
        })?;
        Ok(permissions.into_iter().map(PermissionResponse::from).collect())
    }

    pub async fn get_permission_by_id(&self, id: i32) -> ApplicationResult<Option<PermissionResponse>> {
        let permission_opt = self.repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch permission: {}", e))
        })?;
        Ok(permission_opt.map(PermissionResponse::from))
    }

    pub async fn update_permission(&self, id: i32, req: UpdatePermissionRequest) -> ApplicationResult<()> {
        let mut permission = match self.repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to find permission: {}", e))
        })? {
            Some(p) => p,
            None => return Err(ApplicationError::not_found("Permission not found")),
        };

        if let Some(name) = req.name {
            permission.name = name;
        }
        if let Some(desc) = req.description {
            permission.description = Some(desc);
        }

        permission.validate().map_err(|e| ApplicationError::bad_request(e.to_string()))?;
        self.repo.update(&permission).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to update permission: {}", e))
        })?;
        Ok(())
    }

    pub async fn delete_permission(&self, id: i32) -> ApplicationResult<()> {
        self.repo.delete(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to delete permission: {}", e))
        })
    }
}
