use crate::domain::value_objects::{isbn13::Isbn13, money::Money};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SaleEntity {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub sale_date: DateTime<Utc>,
    pub total_amount: Money,
    pub payment_method: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SaleItemEntity {
    pub id: i32,
    pub sale_id: i32,
    pub book_isbn: Isbn13,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_sale: Money,
    pub subtotal: Money,
    pub created_at: DateTime<Utc>,
}
