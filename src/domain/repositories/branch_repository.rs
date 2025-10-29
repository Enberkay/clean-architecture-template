use async_trait::async_trait;
use crate::domain::entities::branch::BranchEntity;

#[async_trait]
pub trait BranchRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<BranchEntity>, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<BranchEntity>, anyhow::Error>;
    async fn save(&self, branch: &BranchEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}
