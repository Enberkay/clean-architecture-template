use async_trait::async_trait;

use crate::domain::entities::receipt::ReceiptEntity;
use crate::domain::value_objects::receipt_code::ReceiptCode;

#[async_trait]
pub trait ReceiptRepository: Send + Sync {
    async fn find_by_code(&self, code: &ReceiptCode) -> Result<Option<ReceiptEntity>, anyhow::Error>;
    async fn find_by_reference(&self, reference_id: i32) -> Result<Vec<ReceiptEntity>, anyhow::Error>;
    async fn save(&self, receipt: &ReceiptEntity) -> Result<(), anyhow::Error>;
}
