use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct InventoryEntity {
    pub branch_id: i32,
    pub book_isbn: String,
    pub quantity: i32,
    pub updated_at: DateTime<Utc>,
}
