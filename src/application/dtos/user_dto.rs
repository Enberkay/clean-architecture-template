use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::entities::{role::RoleEntity, user::UserEntity};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 2, max = 50, message = "First name must be 2-50 characters"))]
    pub first_name: String,

    #[validate(length(min = 2, max = 50, message = "Last name must be 2-50 characters"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(range(min = 1, max = 120, message = "Age must be between 1 and 120"))]
    pub age: i32,

    #[validate(length(min = 1, max = 20, message = "Sex must be 1-20 characters"))]
    pub sex: String,

    #[validate(length(min = 6, max = 20, message = "Phone must be 6-20 characters"))]
    pub phone: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub branch_id: Option<i32>,
    pub role_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 2, max = 50, message = "First name must be 2-50 characters"))]
    pub first_name: Option<String>,

    #[validate(length(min = 2, max = 50, message = "Last name must be 2-50 characters"))]
    pub last_name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(range(min = 1, max = 120, message = "Age must be between 1 and 120"))]
    pub age: Option<i32>,

    #[validate(length(min = 1, max = 20, message = "Sex must be 1-20 characters"))]
    pub sex: Option<String>,

    #[validate(length(min = 6, max = 20, message = "Phone must be 6-20 characters"))]
    pub phone: Option<String>,

    pub branch_id: Option<i32>,
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
    pub branch_id: Option<i32>,
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
            branch_id: user.branch_id,
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
