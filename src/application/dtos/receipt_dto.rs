use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::domain::entities::receipt::ReceiptEntity;

#[derive(Debug, Deserialize)]
pub struct CreateReceiptRequest {
    pub receipt_code: String,
    pub type_name: String,          // "SALE" | "ORDER"
    pub reference_id: i32,          // order_id or sale_id
    pub source: String,             // "POS" | "ONLINE"
    pub total_amount: f64,
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub payment_method: Option<String>,
    pub payment_ref: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReceiptResponse {
    pub id: i32,
    pub receipt_code: String,
    pub type_name: String,
    pub reference_id: i32,
    pub source: String,
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: f64,
    pub payment_method: Option<String>,
    pub payment_ref: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub status: String,
}

impl From<ReceiptEntity> for ReceiptResponse {
    fn from(entity: ReceiptEntity) -> Self {
        Self {
            id: entity.id,
            receipt_code: entity.receipt_code.as_str().to_string(),
            type_name: entity.type_name,
            reference_id: entity.reference_id,
            source: entity.source,
            user_id: entity.user_id,
            branch_id: entity.branch_id,
            total_amount: entity.total_amount.value().to_f64().unwrap_or(0.0),
            payment_method: entity.payment_method,
            payment_ref: entity.payment_ref,
            issued_at: entity.issued_at,
            status: entity.status,
        }
    }
}
