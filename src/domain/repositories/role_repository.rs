use async_trait::async_trait;

use crate::domain::entities::role::RoleEntity;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<RoleEntity>, anyhow::Error>;
    async fn save(&self, role: &RoleEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}
