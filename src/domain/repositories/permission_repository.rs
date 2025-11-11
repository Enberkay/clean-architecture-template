use async_trait::async_trait;

use crate::domain::entities::permission::PermissionEntity;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn find_all(&self) -> anyhow::Result<Vec<PermissionEntity>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<PermissionEntity>>;
    async fn find_by_ids(&self, ids: &[i32]) -> anyhow::Result<Vec<PermissionEntity>>;
    async fn save(&self, permission: &PermissionEntity) -> anyhow::Result<i32>;
    async fn update(&self, id: i32, name: Option<String>, description: Option<String>) -> anyhow::Result<PermissionEntity>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}
