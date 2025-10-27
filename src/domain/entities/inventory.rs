use crate::domain::value_objects::{isbn13::Isbn13, stock_quantity::StockQuantity};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct InventoryEntity {
    pub branch_id: i32,
    pub book_isbn: Isbn13,
    pub quantity: StockQuantity,
    pub updated_at: DateTime<Utc>,
}

impl InventoryEntity {
    pub fn increase(&mut self, qty: u32) {
        self.quantity.increase(qty);
        self.updated_at = Utc::now();
    }

    pub fn decrease(&mut self, qty: u32) -> Result<(), String> {
        self.quantity.decrease(qty)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_in_stock(&self) -> bool {
        !self.quantity.is_empty()
    }
}
