use std::sync::Arc;
use anyhow::Result;

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

    pub async fn create_permission(&self, req: CreatePermissionRequest) -> Result<PermissionResponse> {
        let permission = PermissionEntity::new(req.name, req.description)?;
        self.repo.save(&permission).await?;
        Ok(PermissionResponse::from(permission))
    }

    pub async fn get_all_permissions(&self) -> Result<Vec<PermissionResponse>> {
        let permissions = self.repo.find_all().await?;
        Ok(permissions.into_iter().map(PermissionResponse::from).collect())
    }

    pub async fn get_permission_by_id(&self, id: i32) -> Result<Option<PermissionResponse>> {
        let permission_opt = self.repo.find_by_id(id).await?;
        Ok(permission_opt.map(PermissionResponse::from))
    }

    pub async fn update_permission(&self, id: i32, req: UpdatePermissionRequest) -> Result<()> {
        let mut permission = match self.repo.find_by_id(id).await? {
            Some(p) => p,
            None => anyhow::bail!("Permission not found"),
        };

        if let Some(name) = req.name {
            permission.name = name;
        }
        if let Some(desc) = req.description {
            permission.description = Some(desc);
        }

        permission.validate()?;
        self.repo.update(&permission).await?;
        Ok(())
    }

    pub async fn delete_permission(&self, id: i32) -> Result<()> {
        self.repo.delete(id).await
    }
}
