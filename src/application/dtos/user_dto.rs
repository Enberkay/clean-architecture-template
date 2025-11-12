use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::{role::RoleEntity, user::UserEntity};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub password: String,
    pub role_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub sex: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct RoleSummary {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub is_active: bool,
    pub roles: Vec<RoleSummary>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserEntity> for UserResponse {
    fn from(user: UserEntity) -> Self {
        Self {
            id: user.id,
            full_name: user.full_name(),
            email: user.email.as_str().to_string(),
            age: user.age,
            sex: user.sex,
            phone: user.phone,
            is_active: user.is_active,
            roles: Vec::new(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<RoleEntity> for RoleSummary {
    fn from(role: RoleEntity) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
        }
    }
}
