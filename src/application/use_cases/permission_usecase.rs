use std::sync::Arc;
use anyhow::{Result, anyhow};
use crate::application::dtos::permission_dto::{
    CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest,
};
use crate::domain::{
    entities::permission::PermissionEntity,
    repositories::permission_repository::PermissionRepository,
};

pub struct PermissionUseCase {
    repo: Arc<dyn PermissionRepository>,
}

impl PermissionUseCase {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_permission(&self, req: CreatePermissionRequest) -> Result<PermissionResponse> {
        let mut permission = PermissionEntity::new(req.name, req.description)
            .map_err(|e| anyhow!(e.to_string()))?;

        let id = self.repo.save(&permission).await.map_err(|e| {
            anyhow!(format!("Failed to save permission: {}", e))
        })?;

        // Set the returned ID to the entity
        permission.id = id;

        Ok(PermissionResponse::from(permission))
    }

    pub async fn get_all_permissions(&self) -> Result<Vec<PermissionResponse>> {
        let permissions = self.repo.find_all().await.map_err(|e| {
            anyhow!(format!("Failed to fetch permissions: {}", e))
        })?;
        Ok(permissions.into_iter().map(PermissionResponse::from).collect())
    }

    pub async fn get_permission_by_id(&self, id: i32) -> Result<Option<PermissionResponse>> {
        let permission_opt = self.repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch permission: {}", e))
        })?;
        Ok(permission_opt.map(PermissionResponse::from))
    }

    pub async fn update_permission(&self, id: i32, req: UpdatePermissionRequest) -> Result<PermissionResponse> {
        // Validate input first
        if let Some(name) = &req.name {
            let temp_permission = PermissionEntity::new(name.clone(), None)
                .map_err(|e| anyhow!(e.to_string()))?;
            temp_permission.validate().map_err(|e| anyhow!(e.to_string()))?;
        }

        // Use COALESCE update - single query approach
        let updated_permission = self.repo.update(
            id,
            req.name,
            req.description
        ).await.map_err(|e| {
            anyhow!(format!("Failed to update permission: {}", e))
        })?;

        Ok(PermissionResponse::from(updated_permission))
    }

    pub async fn delete_permission(&self, id: i32) -> Result<PermissionResponse> {
        // Get permission before deletion for response
        let permission = match self.repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch permission: {}", e))
        })? {
            Some(p) => p,
            None => return Err(anyhow!("Permission not found")),
        };

        // Delete permission
        self.repo.delete(id).await.map_err(|e| {
            anyhow!(format!("Failed to delete permission: {}", e))
        })?;

        // Return deleted permission data
        Ok(PermissionResponse::from(permission))
    }
}
