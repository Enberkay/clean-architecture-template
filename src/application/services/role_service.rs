use std::sync::Arc;
use anyhow::Result;

use crate::application::dtos::role_dto::{
    CreateRoleRequest, RoleResponse, UpdateRoleRequest,
};
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
        Self {
            role_repo,
            perm_repo,
            role_perm_repo,
        }
    }

    pub async fn create_role(&self, req: CreateRoleRequest) -> Result<RoleResponse> {
        let mut role = RoleEntity::new(req.name, req.description)?;

        //save to get valid role.id
        self.role_repo.save(&role).await?;

        //assign permissions if provided
        if let Some(perm_ids) = req.permission_ids {
            let permissions = self.perm_repo.find_by_ids(&perm_ids).await?;
            self.role_perm_repo.assign_permissions(role.id, &perm_ids).await?;
            role.set_permissions(permissions)?;
        }

        Ok(RoleResponse::from(role))
    }

    pub async fn get_all_roles(&self) -> Result<Vec<RoleResponse>> {
        let roles = self.role_repo.find_all().await?;
        Ok(roles.into_iter().map(RoleResponse::from).collect())
    }

    pub async fn get_role_by_id(&self, id: i32) -> Result<Option<RoleResponse>> {
        let role_opt = self.role_repo.find_by_id(id).await?;
        Ok(role_opt.map(RoleResponse::from))
    }

    pub async fn update_role(&self, id: i32, req: UpdateRoleRequest) -> Result<()> {
        let mut role = match self.role_repo.find_by_id(id).await? {
            Some(r) => r,
            None => anyhow::bail!("Role not found"),
        };

        if let Some(name) = req.name {
            role.name = name;
        }
        if let Some(desc) = req.description {
            role.description = Some(desc);
        }

        //Update permissions if specified
        if let Some(perm_ids) = req.permission_ids {
            let permissions = self.perm_repo.find_by_ids(&perm_ids).await?;

            // clear old ones then assign new
            self.role_perm_repo.clear_permissions(role.id).await?;
            self.role_perm_repo.assign_permissions(role.id, &perm_ids).await?;

            role.set_permissions(permissions)?;
        }

        role.validate()?;
        self.role_repo.update(&role).await?;
        Ok(())
    }

    pub async fn delete_role(&self, id: i32) -> Result<()> {
        // optional: remove role-permission links first
        self.role_perm_repo.clear_permissions(id).await?;
        self.role_repo.delete(id).await
    }
}
