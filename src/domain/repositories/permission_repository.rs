use async_trait::async_trait;
use anyhow::Result;

use crate::domain::entities::permission::PermissionEntity;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<PermissionEntity>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<PermissionEntity>>;
    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<PermissionEntity>>;
    async fn save(&self, permission: &PermissionEntity) -> Result<i32>;
    async fn update(&self, permission: &PermissionEntity) -> Result<()>;
    async fn delete(&self, id: i32) -> Result<()>;
}
