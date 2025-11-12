use std::sync::Arc;
use anyhow::{Result, anyhow};
use crate::application::dtos::role_dto::{CreateRoleRequest, RoleResponse, UpdateRoleRequest};
use crate::domain::{
    entities::role::RoleEntity,
    repositories::{
        role_repository::RoleRepository,
        permission_repository::PermissionRepository,
        role_permission_repository::RolePermissionRepository,
    },
};

pub struct RoleUseCase {
    role_repo: Arc<dyn RoleRepository>,
    perm_repo: Arc<dyn PermissionRepository>,
    role_perm_repo: Arc<dyn RolePermissionRepository>,
}

impl RoleUseCase {
    pub fn new(
        role_repo: Arc<dyn RoleRepository>,
        perm_repo: Arc<dyn PermissionRepository>,
        role_perm_repo: Arc<dyn RolePermissionRepository>,
    ) -> Self {
        Self { role_repo, perm_repo, role_perm_repo }
    }

    pub async fn create_role(&self, req: CreateRoleRequest) -> Result<RoleResponse> {
        let mut role = RoleEntity::new(req.name, req.description)
            .map_err(|e| anyhow!(e.to_string()))?;

        let role_id = self.role_repo.save(&role).await.map_err(|e| {
            anyhow!(format!("Failed to save role: {}", e))
        })?;

        if let Some(perm_ids) = req.permission_ids {
            let permissions = self.perm_repo.find_by_ids(&perm_ids).await.map_err(|e| {
                anyhow!(format!("Failed to fetch permissions: {}", e))
            })?;
            self.role_perm_repo.assign_permissions(role_id, &perm_ids).await.map_err(|e| {
                anyhow!(format!("Failed to assign permissions: {}", e))
            })?;
            role.set_permissions(permissions).map_err(|e| {
                anyhow!(e.to_string())
            })?;
        }

        // Set the returned ID to the entity
        role.id = role_id;
        Ok(RoleResponse::from(role))
    }

    pub async fn get_all_roles(&self) -> Result<Vec<RoleResponse>> {
        let roles = self.role_repo.find_all().await.map_err(|e| {
            anyhow!(format!("Failed to fetch roles: {}", e))
        })?;
        Ok(roles.into_iter().map(RoleResponse::from).collect())
    }

    pub async fn get_role_by_id(&self, id: i32) -> Result<Option<RoleResponse>> {
        let role_opt = self.role_repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch role: {}", e))
        })?;
        Ok(role_opt.map(RoleResponse::from))
    }

    pub async fn update_role(&self, id: i32, req: UpdateRoleRequest) -> Result<RoleResponse> {
        // Validate role exists first
        let _ = match self.role_repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch role: {}", e))
        })? {
            Some(r) => r,
            None => return Err(anyhow!("Role not found")),
        };

        // Update role basic info using COALESCE
        let updated_role = self.role_repo.update(
            id,
            req.name,
            req.description
        ).await.map_err(|e| {
            anyhow!(format!("Failed to update role: {}", e))
        })?;

        // Handle permission updates separately if provided
        if let Some(perm_ids) = req.permission_ids {
            self.role_perm_repo.clear_permissions(id).await.map_err(|e| {
                anyhow!(format!("Failed to clear permissions: {}", e))
            })?;
            
            if !perm_ids.is_empty() {
                let permissions = self.perm_repo.find_by_ids(&perm_ids).await.map_err(|e| {
                    anyhow!(format!("Failed to fetch permissions: {}", e))
                })?;
                self.role_perm_repo.assign_permissions(id, &perm_ids).await.map_err(|e| {
                    anyhow!(format!("Failed to assign permissions: {}", e))
                })?;
                
                // Create final role with permissions
                let mut final_role = updated_role;
                final_role.set_permissions(permissions).map_err(|e| anyhow!(e.to_string()))?;
                Ok(RoleResponse::from(final_role))
            } else {
                Ok(RoleResponse::from(updated_role))
            }
        } else {
            // Fetch full role with existing permissions
            match self.role_repo.find_by_id(id).await.map_err(|e| {
                anyhow!(format!("Failed to fetch updated role: {}", e))
            })? {
                Some(full_role) => Ok(RoleResponse::from(full_role)),
                None => Err(anyhow!("Role not found after update")),
            }
        }
    }

    pub async fn delete_role(&self, id: i32) -> Result<RoleResponse> {
        // Get role before deletion for response
        let role = match self.role_repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch role: {}", e))
        })? {
            Some(r) => r,
            None => return Err(anyhow!("Role not found")),
        };

        // Clear permissions first (due to foreign key constraints)
        self.role_perm_repo.clear_permissions(id).await.map_err(|e| {
            anyhow!(format!("Failed to clear permissions: {}", e))
        })?;

        // Delete role
        self.role_repo.delete(id).await.map_err(|e| {
            anyhow!(format!("Failed to delete role: {}", e))
        })?;

        // Return deleted role data
        Ok(RoleResponse::from(role))
    }
}
