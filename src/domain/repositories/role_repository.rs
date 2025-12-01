use async_trait::async_trait;
use crate::domain::entities::role::RoleEntity;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_all(&self) -> anyhow::Result<Vec<RoleEntity>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<RoleEntity>>;
    async fn find_by_name(&self, name: &str) -> anyhow::Result<Option<RoleEntity>>;
    async fn find_by_ids(&self, ids: &[i32]) -> anyhow::Result<Vec<RoleEntity>>;
    async fn save(&self, role: &RoleEntity) -> anyhow::Result<i32>;
    async fn update(&self, role: &RoleEntity) -> anyhow::Result<RoleEntity>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}