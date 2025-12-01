use std::sync::Arc;
use anyhow::{Result, anyhow};

use crate::application::dtos::user_dto::{
    CreateUserRequest, RoleSummary, UpdatePasswordRequest, UpdateUserRequest, UserResponse,
};
use crate::domain::{
    entities::user::UserEntity,
    repositories::{role_repository::RoleRepository, user_repository::UserRepository},
    value_objects::{person_name::PersonName, age::Age},
};
use crate::infrastructure::argon2::PasswordService;

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

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse> {
        if self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            anyhow!("Database error while checking email: {}", e)
        })?.is_some() {
            return Err(anyhow!("Email already exists"));
        }

        let hashed_password = self
            .password_repo
            .hash_password(&req.password)
            .await
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        let mut user = UserEntity::new(
            req.first_name,
            req.last_name,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
        )
        .map_err(|e| anyhow!("{}", e))?;

        let user_id = self
            .user_repo
            .save(&user)
            .await
            .map_err(|e| anyhow!("Failed to save user: {}", e))?;
        user.id = user_id;

        if let Some(role_ids) = req.role_ids.clone() {
            if !role_ids.is_empty() {
                let roles = self.role_repo.find_by_ids(&role_ids).await.map_err(|e| {
                    anyhow!("Failed to fetch roles: {}", e)
                })?;
                if roles.len() != role_ids.len() {
                    return Err(anyhow!("Some roles not found"));
                }
                self.user_repo
                    .assign_roles(user_id, &role_ids)
                    .await
                    .map_err(|e| anyhow!("Failed to assign roles: {}", e))?;
            }
        }

        let roles = self
            .user_repo
            .find_roles(user_id)
            .await
            .map_err(|e| anyhow!("Failed to fetch roles: {}", e))?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<UserResponse>> {
        let user_opt = self.user_repo.find_by_id(id).await.map_err(|e| {
            anyhow!("Database error while fetching user: {}", e)
        })?;

        if let Some(user) = user_opt {
            let roles = self.user_repo.find_roles(id).await.map_err(|e| {
                anyhow!("Failed to fetch user roles: {}", e)
            })?;

            let mut user_response = UserResponse::from(user);
            user_response.roles = roles.into_iter().map(RoleSummary::from).collect();
            Ok(Some(user_response))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>> {
        let users = self.user_repo.find_all().await.map_err(|e| {
            anyhow!("Failed to fetch all users: {}", e)
        })?;

        let mut users_with_roles = Vec::new();
        for user in users {
            let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
                anyhow!("Failed to fetch user roles: {}", e)
            })?;

            let mut user_response = UserResponse::from(user);
            user_response.roles = roles.into_iter().map(RoleSummary::from).collect();
            users_with_roles.push(user_response);
        }

        Ok(users_with_roles)
    }

    pub async fn update_user(&self, id: i32, req: UpdateUserRequest) -> Result<UserResponse> {
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Database error: {}", e))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        if let Some(fname) = req.first_name {
            user.first_name = PersonName::new(fname).map_err(|e| anyhow!("{}", e))?;
        }
        if let Some(lname) = req.last_name {
            user.last_name = PersonName::new(lname).map_err(|e| anyhow!("{}", e))?;
        }
        if let Some(email) = req.email {
            user.update_email(email).map_err(|e| anyhow!("{}", e))?;
        }
        if let Some(age) = req.age {
            user.age = Age::new(age).map_err(|e| anyhow!("{}", e))?;
        }
        if let Some(sex) = req.sex {
            user.sex = sex.trim().to_uppercase();
        }
        if let Some(phone) = req.phone {
            user.update_phone(phone).map_err(|e| anyhow!("{}", e))?;
        }

        let updated_user = self
            .user_repo
            .update(&user)
            .await
            .map_err(|e| anyhow!("Failed to update user: {}", e))?;

        let roles = self.user_repo.find_roles(id).await.map_err(|e| {
            anyhow!("Failed to fetch user roles: {}", e)
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    pub async fn delete_user(&self, id: i32) -> Result<UserResponse> {
        let user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Failed to fetch user: {}", e))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        let roles = self.user_repo.find_roles(id).await.map_err(|e| {
            anyhow!("Failed to fetch user roles: {}", e)
        })?;

        self.user_repo
            .delete(id)
            .await
            .map_err(|e| anyhow!("Failed to delete user: {}", e))?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    pub async fn assign_roles(&self, user_id: i32, role_ids: Vec<i32>) -> Result<()> {
        let user_opt = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| anyhow!("Failed to fetch user: {}", e))?;

        if user_opt.is_none() {
            return Err(anyhow!("User not found"));
        }

        let roles = self
            .role_repo
            .find_by_ids(&role_ids)
            .await
            .map_err(|e| anyhow!("Failed to fetch roles: {}", e))?;

        if roles.len() != role_ids.len() {
            return Err(anyhow!("Some roles not found"));
        }

        self.user_repo
            .assign_roles(user_id, &role_ids)
            .await
            .map_err(|e| anyhow!("Failed to assign roles: {}", e))
    }

    pub async fn remove_roles(&self, user_id: i32, role_ids: Vec<i32>) -> Result<()> {
        self.user_repo
            .remove_roles(user_id, &role_ids)
            .await
            .map_err(|e| anyhow!("Failed to remove roles: {}", e))
    }

    pub async fn get_user_roles(&self, user_id: i32) -> Result<Vec<String>> {
        let roles = self.user_repo.find_roles(user_id).await.map_err(|e| {
            anyhow!("Failed to fetch user roles: {}", e)
        })?;
        Ok(roles.into_iter().map(|r| r.name.as_str().to_string()).collect())
    }

    pub async fn deactivate_user(&self, id: i32) -> Result<UserResponse> {
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Failed to fetch user: {}", e))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        user.deactivate();

        let updated_user = self
            .user_repo
            .update(&user)
            .await
            .map_err(|e| anyhow!("Failed to deactivate user: {}", e))?;

        let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
            anyhow!("Failed to fetch user roles: {}", e)
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    pub async fn activate_user(&self, id: i32) -> Result<UserResponse> {
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Failed to fetch user: {}", e))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        user.activate();

        let updated_user = self
            .user_repo
            .update(&user)
            .await
            .map_err(|e| anyhow!("Failed to activate user: {}", e))?;

        let roles = self.user_repo.find_roles(user.id).await.map_err(|e| {
            anyhow!("Failed to fetch user roles: {}", e)
        })?;

        let mut user_response = UserResponse::from(updated_user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }

    pub async fn update_password(
        &self,
        id: i32,
        req: UpdatePasswordRequest,
    ) -> Result<UserResponse> {
        let mut user = match self
            .user_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!("Failed to fetch user: {}", e))?
        {
            Some(u) => u,
            None => return Err(anyhow!("User not found")),
        };

        let hashed = self
            .password_repo
            .hash_password(&req.new_password)
            .await
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        user.change_password(hashed).map_err(|e| anyhow!("{}", e))?;

        self.user_repo
            .update_password(user.id, user.password.as_str())
            .await
            .map_err(|e| anyhow!("Failed to update password: {}", e))?;

        let roles = self
            .user_repo
            .find_roles(user.id)
            .await
            .map_err(|e| anyhow!("Failed to fetch roles: {}", e))?;

        let mut user_response = UserResponse::from(user);
        user_response.roles = roles.into_iter().map(RoleSummary::from).collect();

        Ok(user_response)
    }
}