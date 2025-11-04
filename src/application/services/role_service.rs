use std::sync::Arc;
use crate::application::application_errors::{ApplicationError, ApplicationResult};
use crate::application::dtos::role_dto::{CreateRoleRequest, RoleResponse, UpdateRoleRequest};
use crate::domain::{
    entities::role::RoleEntity,
    repositories::{
        role_repository::RoleRepository,
        permission_repository::PermissionRepository,
        role_permission_repository::RolePermissionRepository,
    },
};

pub struct RoleService {
    role_repo: Arc<dyn RoleRepository>,
    perm_repo: Arc<dyn PermissionRepository>,
    role_perm_repo: Arc<dyn RolePermissionRepository>,
}

impl RoleService {
    pub fn new(
        role_repo: Arc<dyn RoleRepository>,
        perm_repo: Arc<dyn PermissionRepository>,
        role_perm_repo: Arc<dyn RolePermissionRepository>,
    ) -> Self {
        Self { role_repo, perm_repo, role_perm_repo }
    }

    pub async fn create_role(&self, req: CreateRoleRequest) -> ApplicationResult<RoleResponse> {
        let mut role = RoleEntity::new(req.name, req.description)
            .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        self.role_repo.save(&role).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to save role: {}", e))
        })?;

        if let Some(perm_ids) = req.permission_ids {
            let permissions = self.perm_repo.find_by_ids(&perm_ids).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to fetch permissions: {}", e))
            })?;
            self.role_perm_repo.assign_permissions(role.id, &perm_ids).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to assign permissions: {}", e))
            })?;
            role.set_permissions(permissions).map_err(|e| {
                ApplicationError::bad_request(e.to_string())
            })?;
        }

        Ok(RoleResponse::from(role))
    }

    pub async fn get_all_roles(&self) -> ApplicationResult<Vec<RoleResponse>> {
        let roles = self.role_repo.find_all().await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch roles: {}", e))
        })?;
        Ok(roles.into_iter().map(RoleResponse::from).collect())
    }

    pub async fn get_role_by_id(&self, id: i32) -> ApplicationResult<Option<RoleResponse>> {
        let role_opt = self.role_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch role: {}", e))
        })?;
        Ok(role_opt.map(RoleResponse::from))
    }

    pub async fn update_role(&self, id: i32, req: UpdateRoleRequest) -> ApplicationResult<()> {
        let mut role = match self.role_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch role: {}", e))
        })? {
            Some(r) => r,
            None => return Err(ApplicationError::not_found("Role not found")),
        };

        if let Some(name) = req.name {
            role.name = name;
        }
        if let Some(desc) = req.description {
            role.description = Some(desc);
        }

        if let Some(perm_ids) = req.permission_ids {
            let permissions = self.perm_repo.find_by_ids(&perm_ids).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to fetch permissions: {}", e))
            })?;
            self.role_perm_repo.clear_permissions(role.id).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to clear permissions: {}", e))
            })?;
            self.role_perm_repo.assign_permissions(role.id, &perm_ids).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to assign permissions: {}", e))
            })?;
            role.set_permissions(permissions).map_err(|e| ApplicationError::bad_request(e.to_string()))?;
        }

        role.validate().map_err(|e| ApplicationError::bad_request(e.to_string()))?;
        self.role_repo.update(&role).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to update role: {}", e))
        })?;
        Ok(())
    }

    pub async fn delete_role(&self, id: i32) -> ApplicationResult<()> {
        self.role_perm_repo.clear_permissions(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to clear permissions: {}", e))
        })?;
        self.role_repo.delete(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to delete role: {}", e))
        })
    }
}
