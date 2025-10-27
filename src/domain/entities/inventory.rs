use crate::domain::{
    domain_errors::{DomainError, DomainResult},
    value_objects::{isbn13::Isbn13, stock_quantity::StockQuantity},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct InventoryEntity {
    pub branch_id: i32,
    pub book_isbn: Isbn13,
    pub quantity: StockQuantity,
    pub updated_at: DateTime<Utc>,
}

impl InventoryEntity {
    /// Create new inventory record
    pub fn new(branch_id: i32, book_isbn: Isbn13, quantity: i32) -> DomainResult<Self> {
        if branch_id <= 0 {
            return Err(DomainError::validation("Branch ID must be positive"));
        }

        let qty = StockQuantity::new(quantity)?;
        Ok(Self {
            branch_id,
            book_isbn,
            quantity: qty,
            updated_at: Utc::now(),
        })
    }

    /// Increase stock by given quantity
    pub fn increase(&mut self, qty: u32) -> DomainResult<()> {
        if qty == 0 {
            return Err(DomainError::validation("Increase amount must be greater than zero"));
        }
        self.quantity.increase(qty);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Decrease stock by given quantity
    pub fn decrease(&mut self, qty: u32) -> DomainResult<()> {
        if qty == 0 {
            return Err(DomainError::validation("Decrease amount must be greater than zero"));
        }
        self.quantity.decrease(qty)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Replace stock quantity directly (used by sync jobs)
    pub fn set_quantity(&mut self, qty: i32) -> DomainResult<()> {
        self.quantity = StockQuantity::new(qty)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Returns `true` if there is at least one item in stock
    pub fn is_in_stock(&self) -> bool {
        !self.quantity.is_empty()
    }

    /// Returns current stock count
    pub fn available_quantity(&self) -> u32 {
        self.quantity.value()
    }
}
