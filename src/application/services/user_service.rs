use std::sync::Arc;
use validator::Validate;

use crate::application::application_errors::{ApplicationError, ApplicationResult};
use crate::application::dtos::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse, RoleSummary};
use crate::domain::{
    entities::user::UserEntity,
    repositories::{
        role_repository::RoleRepository,
        user_repository::UserRepository,
        password_repository::PasswordRepository,
    },
};

/// UserService — encapsulates application-level business logic
/// for managing users (create, read, update, delete, and role assignment)
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
    password_repo: Arc<dyn PasswordRepository>,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
        password_repo: Arc<dyn PasswordRepository>,
    ) -> Self {
        Self { user_repo, role_repo, password_repo }
    }

    /// Create a new user — now hashes password & supports role assignment
    pub async fn create_user(&self, req: CreateUserRequest) -> ApplicationResult<UserResponse> {
        // Validate DTO
        req.validate()
            .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        // ตรวจสอบ email ซ้ำ
        if let Some(_) = self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            ApplicationError::internal(format!("Database error while checking email: {}", e))
        })? {
            return Err(ApplicationError::conflict("Email already exists"));
        }

        // Hash password ก่อน save
        let hashed_password = self.password_repo.hash(&req.password).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to hash password: {}", e))
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
            req.branch_id,
        )
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        // Save user
        let user_id = self.user_repo.save(&user).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to save user: {}", e))
        })?;
        user.id = user_id;

        // Assign roles ถ้ามี role_ids
        if let Some(role_ids) = req.role_ids.clone() {
            if !role_ids.is_empty() {
                let roles = self.role_repo.find_by_ids(&role_ids).await.map_err(|e| {
                    ApplicationError::internal(format!("Failed to fetch roles: {}", e))
                })?;
                if roles.len() != role_ids.len() {
                    return Err(ApplicationError::bad_request("Some roles not found".to_string()));
                }
                self.user_repo.assign_roles(user_id, &role_ids).await.map_err(|e| {
                    ApplicationError::internal(format!("Failed to assign roles: {}", e))
                })?;
            }
        }

        // โหลด roles กลับมาใน response
        let roles = self.user_repo.find_roles(user_id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch roles: {}", e))
        })?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: i32) -> ApplicationResult<Option<UserResponse>> {
        let user_opt = self.user_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Database error while fetching user: {}", e))
        })?;

        if let Some(user) = user_opt {
            let roles = self.user_repo.find_roles(id).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to fetch user roles: {}", e))
            })?;

            let mut user_response = UserResponse::from(user);
            user_response.roles = roles.into_iter().map(RoleSummary::from).collect();
            Ok(Some(user_response))
        } else {
            Ok(None)
        }
    }

    /// Get all users
    pub async fn get_all_users(&self) -> ApplicationResult<Vec<UserResponse>> {
        let users = self.user_repo.find_all().await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch all users: {}", e))
        })?;

        let mut users_with_roles = Vec::new();
        for user in users {
            let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
                ApplicationError::internal(format!("Failed to fetch user roles: {}", e))
            })?;

            let mut user_response = UserResponse::from(user);
            user_response.roles = roles.into_iter().map(RoleSummary::from).collect();
            users_with_roles.push(user_response);
        }

        Ok(users_with_roles)
    }

    /// Update user profile (except password)
    pub async fn update_user(&self, id: i32, req: UpdateUserRequest) -> ApplicationResult<UserResponse> {
        // ตรวจว่าผู้ใช้มีอยู่จริง
        let _ = match self.user_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Database error: {}", e))
        })? {
            Some(u) => u,
            None => return Err(ApplicationError::not_found("User not found")),
        };

        // Validate DTO
        req.validate()
            .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        let normalized_sex = req.sex.map(|s| s.to_uppercase());

        // Update แบบ COALESCE
        let updated_user = self.user_repo.update(
            id,
            req.first_name,
            req.last_name,
            req.email,
            req.age,
            normalized_sex,
            req.phone,
            req.branch_id,
            None,
        ).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to update user: {}", e))
        })?;

        let roles = self.user_repo.find_roles(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user roles: {}", e))
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    /// Delete user
    pub async fn delete_user(&self, id: i32) -> ApplicationResult<UserResponse> {
        let user = match self.user_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user: {}", e))
        })? {
            Some(u) => u,
            None => return Err(ApplicationError::not_found("User not found")),
        };

        let roles = self.user_repo.find_roles(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user roles: {}", e))
        })?;

        self.user_repo.delete(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to delete user: {}", e))
        })?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    // =====================================================
    // RBAC FEATURES
    // =====================================================

    pub async fn assign_roles(&self, user_id: i32, role_ids: Vec<i32>) -> ApplicationResult<()> {
        let user_opt = self.user_repo.find_by_id(user_id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user: {}", e))
        })?;

        if user_opt.is_none() {
            return Err(ApplicationError::not_found("User not found"));
        }

        let roles = self.role_repo.find_by_ids(&role_ids).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch roles: {}", e))
        })?;

        if roles.len() != role_ids.len() {
            return Err(ApplicationError::bad_request("Some roles not found".to_string()));
        }

        self.user_repo.assign_roles(user_id, &role_ids).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to assign roles: {}", e))
        })
    }

    pub async fn remove_roles(&self, user_id: i32, role_ids: Vec<i32>) -> ApplicationResult<()> {
        self.user_repo.remove_roles(user_id, &role_ids).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to remove roles: {}", e))
        })
    }

    pub async fn get_user_roles(&self, user_id: i32) -> ApplicationResult<Vec<String>> {
        let roles = self.user_repo.find_roles(user_id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user roles: {}", e))
        })?;
        Ok(roles.into_iter().map(|r| r.name).collect())
    }
}
