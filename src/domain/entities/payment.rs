use crate::domain::value_objects::money::Money;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PaymentEntity {
    pub id: i32,
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub payment_method: String,
    pub transaction_ref: Option<String>,
    pub amount: Money,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
