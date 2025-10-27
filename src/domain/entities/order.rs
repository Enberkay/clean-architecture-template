use crate::domain::value_objects::money::Money;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct OrderEntity {
    pub id: i32,
    pub user_id: Option<i32>,
    pub order_date: DateTime<Utc>,
    pub status: String, // e.g. PENDING, PAID, SHIPPED
    pub source: String, // e.g. ONLINE
    pub total_amount: Money,
    pub shipping_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct OrderItemEntity {
    pub id: i32,
    pub order_id: i32,
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_purchase: Money,
    pub subtotal: Money,
    pub created_at: DateTime<Utc>,
}
