use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ReceiptEntity {
    pub id: i32,
    pub receipt_code: String,
    pub r#type: String,    // SALE or ORDER
    pub reference_id: i32, // FK: sales.id or orders.id
    pub source: String,    // POS or ONLINE
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: f64,
    pub payment_method: Option<String>,
    pub payment_ref: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
