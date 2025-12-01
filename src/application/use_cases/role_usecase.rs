use std::sync::Arc;
use anyhow::{Result, anyhow};

use crate::application::dtos::role_dto::{CreateRoleRequest, UpdateRoleRequest, RoleResponse};
use crate::domain::{
    entities::role::RoleEntity,
    repositories::role_repository::RoleRepository,
};

/// RoleUseCase — encapsulates application-level business logic for managing roles
pub struct RoleUseCase {
    role_repo: Arc<dyn RoleRepository>,
}

impl RoleUseCase {
    pub fn new(role_repo: Arc<dyn RoleRepository>) -> Self {
        Self { role_repo }
    }

    /// Create a new role
    pub async fn create_role(&self, req: CreateRoleRequest) -> Result<RoleResponse> {
        // 1. ตรวจสอบว่าชื่อซ้ำไหม (Duplicate Check)
        if self.role_repo.find_by_name(&req.name).await.map_err(|e| {
            anyhow!("Database error while checking role name: {}", e)
        })?.is_some() {
            return Err(anyhow!("Role name '{}' already exists", req.name));
        }

        // 2. สร้าง entity (Validation เกิดขึ้นใน RoleEntity::new -> Value Objects)
        let mut role = RoleEntity::new(req.name, req.description)
            .map_err(|e| anyhow!("{}", e))?;

        // 3. Save role
        let role_id = self
            .role_repo
            .save(&role)
            .await
            .map_err(|e| anyhow!("Failed to save role: {}", e))?;

        role.id = role_id;

        Ok(RoleResponse::from(role))
    }

    /// Get role by ID
    pub async fn get_role_by_id(&self, id: i32) -> Result<Option<RoleResponse>> {
        let role_opt = self.role_repo.find_by_id(id).await.map_err(|e| {
            anyhow!("Database error while fetching role: {}", e)
        })?;

        Ok(role_opt.map(RoleResponse::from))
    }

    /// Get all roles
    pub async fn get_all_roles(&self) -> Result<Vec<RoleResponse>> {
        let roles = self.role_repo.find_all().await.map_err(|e| {
            anyhow!("Failed to fetch all roles: {}", e)
        })?;

        Ok(roles.into_iter().map(RoleResponse::from).collect())
    }

    /// Update role
    pub async fn update_role(
        &self,
        id: i32,
        req: UpdateRoleRequest,
    ) -> Result<RoleResponse> {
        // 1. Fetch Entity เดิมออกมาก่อน
        let mut role = match self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Database error: {}", e))?
        {
            Some(r) => r,
            None => return Err(anyhow!("Role not found")),
        };

        // 2. Handle Name Update
        if let Some(new_name) = req.name {
            // ถ้าชื่อเปลี่ยน ให้เช็คว่าชื่อใหม่ไปซ้ำกับคนอื่นไหม
            if role.name.as_str() != new_name {
                if self.role_repo.find_by_name(&new_name).await.map_err(|e| {
                    anyhow!("Database error while checking role name: {}", e)
                })?.is_some() {
                    return Err(anyhow!("Role name '{}' already exists", new_name));
                }

                // สั่ง Rename (Validation ใน ValueObject จะทำงาน)
                role.rename(new_name).map_err(|e| anyhow!("{}", e))?;
            }
        }

        // 3. Handle Description Update
        if let Some(desc_str) = req.description {
            // Logic: ถ้าส่ง empty string มา ให้ถือว่าเป็น None (ลบ description)
            let desc_opt = if desc_str.trim().is_empty() {
                None
            } else {
                Some(desc_str)
            };
            
            role.update_description(desc_opt).map_err(|e| anyhow!("{}", e))?;
        }

        // 4. Save Changes (ส่ง Entity ทั้งก้อนไป update)
        let updated_role = self
            .role_repo
            .update(&role)
            .await
            .map_err(|e| anyhow!("Failed to update role: {}", e))?;

        Ok(RoleResponse::from(updated_role))
    }

    /// Delete role
    pub async fn delete_role(&self, id: i32) -> Result<RoleResponse> {
        let role = match self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Failed to fetch role: {}", e))?
        {
            Some(r) => r,
            None => return Err(anyhow!("Role not found")),
        };

        // อาจเพิ่ม Logic เช็คว่ามี User ใช้งาน Role นี้อยู่ไหมก่อนลบได้ตรงนี้

        self.role_repo
            .delete(id)
            .await
            .map_err(|e| anyhow!("Failed to delete role: {}", e))?;

        Ok(RoleResponse::from(role))
    }
}