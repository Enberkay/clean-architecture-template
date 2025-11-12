use std::sync::Arc;

use anyhow::{Result, anyhow};

use crate::application::dtos::user_dto::{
    CreateUserRequest, RoleSummary, UpdatePasswordRequest, UpdateUserRequest, UserResponse,
};
use crate::{
    domain::{
        entities::user::UserEntity,
        repositories::{role_repository::RoleRepository, user_repository::UserRepository},
    },
    infrastructure::security::argon2::PasswordService,
};

/// UserUseCase — encapsulates application-level business logic
/// for managing users (create, read, update, delete, and role assignment)
pub struct UserUseCase {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
    password_repo: Arc<dyn PasswordService>,
}

impl UserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        password_repo: Arc<dyn PasswordService>,
    ) -> Self {
        Self {
            user_repo,
            role_repo,
            password_repo,
        }
    }

    /// Create a new user — now hashes password & supports role assignment
    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse> {


        // Check for duplicate email
        if let Some(_) = self
            .user_repo
            .find_by_email(&req.email)
            .await
            .map_err(|e| {
                anyhow!(format!("Database error while checking email: {}", e))
            })?
        {
            return Err(anyhow!("Email already exists"));
        }

        // Hash password ก่อน save
        let hashed_password =
            self.password_repo.hash_password(&req.password).await.map_err(|e| {
                anyhow!(format!("Failed to hash password: {}", e))
            })?;

        // สร้าง entity
        let mut user = UserEntity::new(
            req.first_name,
            req.last_name,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
        )
        .map_err(|e| anyhow!(e.to_string()))?;

        // Save user
        let user_id = self
            .user_repo
            .save(&user)
            .await
            .map_err(|e| anyhow!(format!("Failed to save user: {}", e)))?;
        user.id = user_id;

        // Assign roles ถ้ามี role_ids
        if let Some(role_ids) = req.role_ids.clone() {
            if !role_ids.is_empty() {
                let roles = self.role_repo.find_by_ids(&role_ids).await.map_err(|e| {
                    anyhow!(format!("Failed to fetch roles: {}", e))
                })?;
                if roles.len() != role_ids.len() {
                    return Err(anyhow!(
                        "Some roles not found".to_string(),
                    ));
                }
                self.user_repo
                    .assign_roles(user_id, &role_ids)
                    .await
                    .map_err(|e| {
                        anyhow!(format!("Failed to assign roles: {}", e))
                    })?;
            }
        }

        // โหลด roles กลับมาใน response
        let roles = self
            .user_repo
            .find_roles(user_id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch roles: {}", e)))?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<UserResponse>> {
        let user_opt = self.user_repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Database error while fetching user: {}", e))
        })?;

        if let Some(user) = user_opt {
            let roles = self.user_repo.find_roles(id).await.map_err(|e| {
                anyhow!(format!("Failed to fetch user roles: {}", e))
            })?;

            let mut user_response = UserResponse::from(user);
            user_response.roles = roles.into_iter().map(RoleSummary::from).collect();
            Ok(Some(user_response))
        } else {
            Ok(None)
        }
    }

    /// Get all users
    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>> {
        let users =
            self.user_repo.find_all().await.map_err(|e| {
                anyhow!(format!("Failed to fetch all users: {}", e))
            })?;

        let mut users_with_roles = Vec::new();
        for user in users {
            let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
                anyhow!(format!("Failed to fetch user roles: {}", e))
            })?;

            let mut user_response = UserResponse::from(user);
            user_response.roles = roles.into_iter().map(RoleSummary::from).collect();
            users_with_roles.push(user_response);
        }

        Ok(users_with_roles)
    }

    /// Update user profile (except password)
    pub async fn update_user(
        &self,
        id: i32,
        req: UpdateUserRequest,
    ) -> Result<UserResponse> {
        // ตรวจว่าผู้ใช้มีอยู่จริง
        let _ = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Database error: {}", e)))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };



        let normalized_sex = req.sex.map(|s| s.to_uppercase());

        // Update แบบ COALESCE
        let updated_user = self
            .user_repo
            .update(
                id,
                req.first_name,
                req.last_name,
                req.email,
                req.age,
                normalized_sex,
                req.phone,
                None,
            )
            .await
            .map_err(|e| anyhow!(format!("Failed to update user: {}", e)))?;

        let roles = self.user_repo.find_roles(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch user roles: {}", e))
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    /// Delete user
    pub async fn delete_user(&self, id: i32) -> Result<UserResponse> {
        let user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch user: {}", e)))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        let roles = self.user_repo.find_roles(id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch user roles: {}", e))
        })?;

        self.user_repo
            .delete(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to delete user: {}", e)))?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    // =====================================================
    // RBAC FEATURES
    // =====================================================

    pub async fn assign_roles(&self, user_id: i32, role_ids: Vec<i32>) -> Result<()> {
        let user_opt = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch user: {}", e)))?;

        if user_opt.is_none() {
            return Err(anyhow!("User not found"));
        }

        let roles = self
            .role_repo
            .find_by_ids(&role_ids)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch roles: {}", e)))?;

        if roles.len() != role_ids.len() {
            return Err(anyhow!(
                "Some roles not found".to_string(),
            ));
        }

        self.user_repo
            .assign_roles(user_id, &role_ids)
            .await
            .map_err(|e| anyhow!(format!("Failed to assign roles: {}", e)))
    }

    pub async fn remove_roles(&self, user_id: i32, role_ids: Vec<i32>) -> Result<()> {
        self.user_repo
            .remove_roles(user_id, &role_ids)
            .await
            .map_err(|e| anyhow!(format!("Failed to remove roles: {}", e)))
    }

    pub async fn get_user_roles(&self, user_id: i32) -> Result<Vec<String>> {
        let roles = self.user_repo.find_roles(user_id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch user roles: {}", e))
        })?;
        Ok(roles.into_iter().map(|r| r.name).collect())
    }

    /// Deactivate user (soft delete)
    pub async fn deactivate_user(&self, id: i32) -> Result<UserResponse> {
        // ตรวจว่าผู้ใช้มีอยู่จริง
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch user: {}", e)))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        // เปลี่ยนสถานะใน domain entity
        user.deactivate();

        // อัปเดตใน database
        let updated_user = self
            .user_repo
            .update(
                user.id,
                Some(user.first_name.clone()),
                Some(user.last_name.clone()),
                Some(user.email.as_str().to_string()),
                Some(user.age),
                Some(user.sex.clone()),
                Some(user.phone.clone()),
                Some(user.is_active), // <- is_active = false
            )
            .await
            .map_err(|e| anyhow!(format!("Failed to deactivate user: {}", e)))?;

        // โหลด roles กลับมาใน response
        let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch user roles: {}", e))
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    /// Reactivate user (soft restore)
    pub async fn activate_user(&self, id: i32) -> Result<UserResponse> {
        // ตรวจว่าผู้ใช้มีอยู่จริง
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch user: {}", e)))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        // เปลี่ยนสถานะใน domain entity
        user.activate();

        // อัปเดตใน database
        let updated_user = self
            .user_repo
            .update(
                user.id,
                Some(user.first_name.clone()),
                Some(user.last_name.clone()),
                Some(user.email.as_str().to_string()),
                Some(user.age),
                Some(user.sex.clone()),
                Some(user.phone.clone()),
                Some(user.is_active), // <- true
            )
            .await
            .map_err(|e| anyhow!(format!("Failed to activate user: {}", e)))?;

        // โหลด roles กลับมาใน response
        let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
            anyhow!(format!("Failed to fetch user roles: {}", e))
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    /// Change user's password (re-hash)
    pub async fn update_password(
        &self,
        id: i32,
        req: UpdatePasswordRequest,
    ) -> Result<UserResponse> {


        // หา user ก่อน
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch user: {}", e)))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        // Hash password ใหม่
        let hashed = self
            .password_repo
            .hash_password(&req.new_password)
            .await
            .map_err(|e| anyhow!(format!("Failed to hash password: {}", e)))?;

        // อัปเดตใน domain
        user.change_password(hashed)
            .map_err(|e| anyhow!(e.to_string()))?;

        // Update เฉพาะ password + updated_at
        let updated_user = self
            .user_repo
            .update(
                user.id,
                Some(user.first_name.clone()),
                Some(user.last_name.clone()),
                Some(user.email.as_str().to_string()),
                Some(user.age),
                Some(user.sex.clone()),
                Some(user.phone.clone()),
                Some(user.is_active),
            )
            .await
            .map_err(|e| anyhow!(format!("Failed to update password: {}", e)))?;

        let roles = self
            .user_repo
            .find_roles(user.id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch roles: {}", e)))?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }
}
