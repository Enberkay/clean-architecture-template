use crate::domain::value_objects::{isbn13::Isbn13, stock_quantity::StockQuantity};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct InventoryEntity {
    pub branch_id: i32,
    pub book_isbn: Isbn13,
    pub quantity: StockQuantity,
    pub updated_at: DateTime<Utc>,
}
