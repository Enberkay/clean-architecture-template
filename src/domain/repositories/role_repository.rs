use async_trait::async_trait;
use crate::domain::entities::role::RoleEntity;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<RoleEntity>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<RoleEntity>>;
    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<RoleEntity>>;
    async fn save(&self, role: &RoleEntity) -> Result<i32>;
    async fn update(&self, id: i32, name: Option<String>, description: Option<String>) -> Result<RoleEntity>;
    async fn delete(&self, id: i32) -> Result<()>;
}
