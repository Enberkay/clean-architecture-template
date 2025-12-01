use async_trait::async_trait;
use crate::domain::entities::{user::UserEntity, role::RoleEntity};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> anyhow::Result<Vec<UserEntity>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<UserEntity>>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;
    async fn save(&self, user: &UserEntity) -> anyhow::Result<i32>;
    async fn update(&self, user: &UserEntity) -> anyhow::Result<UserEntity>;
    async fn update_password(&self, id: i32, new_password_hash: &str) -> anyhow::Result<()>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
    
    // RBAC
    async fn assign_roles(&self, user_id: i32, role_ids: &[i32]) -> anyhow::Result<()>;
    async fn remove_roles(&self, user_id: i32, role_ids: &[i32]) -> anyhow::Result<()>;
    async fn find_roles(&self, user_id: i32) -> anyhow::Result<Vec<RoleEntity>>;
}