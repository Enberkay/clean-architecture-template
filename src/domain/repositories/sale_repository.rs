use crate::domain::entities::sale::{SaleEntity, SaleItemEntity};
use async_trait::async_trait;

#[async_trait]
pub trait SaleRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<SaleEntity>, anyhow::Error>;
    async fn find_items(&self, sale_id: i32) -> Result<Vec<SaleItemEntity>, anyhow::Error>;
    async fn save(&self, sale: &SaleEntity) -> Result<(), anyhow::Error>;
    async fn save_items(&self, items: &[SaleItemEntity]) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
    async fn save_with_items(&self, sale: &SaleEntity, items: &[SaleItemEntity]) -> anyhow::Result<()> {
        self.save(sale).await?;
        self.save_items(items).await?;
        Ok(())
    }
}
