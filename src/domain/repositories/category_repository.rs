use crate::domain::entities::category::CategoryEntity;
use async_trait::async_trait;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<CategoryEntity>, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<CategoryEntity>, anyhow::Error>;
    async fn save(&self, category: &CategoryEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}
