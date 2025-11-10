use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StockQuantity(u32);

impl StockQuantity {
    /// Creates a new StockQuantity, ensuring it's non-negative.
    pub fn new(value: i32) -> Result<Self> {
        if value < 0 {
            return Err(anyhow!("Stock quantity cannot be negative"));
        }
        Ok(Self(value as u32))
    }

    /// Get the inner value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Increase stock by amount.
    pub fn increase(&mut self, amount: u32) {
        self.0 = self.0.saturating_add(amount);
    }

    /// Decrease stock safely. Returns Err if insufficient.
    pub fn decrease(&mut self, amount: u32) -> Result<()> {
        if amount > self.0 {
            return Err(anyhow!("Insufficient stock quantity"));
        }
        self.0 -= amount;
        Ok(())
    }

    /// Check if stock is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Reset stock to zero.
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}
