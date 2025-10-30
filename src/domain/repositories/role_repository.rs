use async_trait::async_trait;
use anyhow::Result;

use crate::domain::entities::role::RoleEntity;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<RoleEntity>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<RoleEntity>>;
    async fn save(&self, role: &RoleEntity) -> Result<()>;
    async fn update(&self, role: &RoleEntity) -> Result<()>;
    async fn delete(&self, id: i32) -> Result<()>;
}
