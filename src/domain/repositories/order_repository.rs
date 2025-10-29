use crate::domain::entities::order::{OrderEntity, OrderItemEntity};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository: Send + Sync {
   async fn find_by_id(&self, id: i32) -> Result<Option<OrderEntity>, anyhow::Error>;
   async fn find_items(&self, order_id: i32) -> Result<Vec<OrderItemEntity>, anyhow::Error>;
   async fn save(&self, order: &OrderEntity) -> Result<(), anyhow::Error>;
   async fn save_items(&self, items: &[OrderItemEntity]) -> Result<(), anyhow::Error>;
   async fn delete(&self, id: i32) -> Result<()>;
   async fn save_with_items(&self, order: &OrderEntity, items: &[OrderItemEntity]) -> Result<()> {
            self.save(order).await?;
            self.save_items(items).await?;
            Ok(())
        }
}
