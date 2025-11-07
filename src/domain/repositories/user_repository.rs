use async_trait::async_trait;
use anyhow::Result;

use crate::domain::entities::{user::UserEntity, role::RoleEntity};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<UserEntity>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>>;
    async fn save(&self, user: &UserEntity) -> Result<i32>;
    async fn update(&self, id: i32, first_name: Option<String>, last_name: Option<String>, email: Option<String>, age: Option<i32>, sex: Option<String>, phone: Option<String>, branch_id: Option<i32>, is_active: Option<bool>) -> Result<UserEntity>;
    async fn delete(&self, id: i32) -> Result<()>;

    // RBAC
    async fn assign_roles(&self, user_id: i32, role_ids: &[i32]) -> Result<()>;
    async fn remove_roles(&self, user_id: i32, role_ids: &[i32]) -> Result<()>;
    async fn find_roles(&self, user_id: i32) -> Result<Vec<RoleEntity>>;
}
