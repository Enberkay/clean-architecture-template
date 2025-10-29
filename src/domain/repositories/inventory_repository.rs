use async_trait::async_trait;

use crate::domain::entities::inventory::InventoryEntity;
use crate::domain::value_objects::isbn13::Isbn13;

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn find(&self, branch_id: i32, isbn: &Isbn13)-> Result<Option<InventoryEntity>, anyhow::Error>;
    async fn save(&self, inventory: &InventoryEntity) -> Result<(), anyhow::Error>;
    async fn update(&self, inventory: &InventoryEntity) -> Result<(), anyhow::Error>;
}
