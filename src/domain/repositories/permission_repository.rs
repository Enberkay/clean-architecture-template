use async_trait::async_trait;

use crate::domain::entities::permission::PermissionEntity;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PermissionEntity>, anyhow::Error>;
    async fn save(&self, permission: &PermissionEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}
