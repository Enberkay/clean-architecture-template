use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::domain::entities::{sale::SaleEntity, payment::PaymentEntity, receipt::ReceiptEntity};

#[derive(Debug, Deserialize)]
pub struct CreateSaleRequest {
    pub user_id: i32,
    pub branch_id: i32,
    pub total_amount: f64,
    pub payment_method: String, // e.g. "CASH", "PROMPTPAY"
}

#[derive(Debug, Serialize)]
pub struct SaleResponse {
    pub sale_id: i32,
    pub total_amount: f64,
    pub payment_status: String,
    pub payment_ref: Option<String>,
    pub receipt_code: Option<String>,
    pub receipt_status: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<(SaleEntity, PaymentEntity, ReceiptEntity)> for SaleResponse {
    fn from((sale, payment, receipt): (SaleEntity, PaymentEntity, ReceiptEntity)) -> Self {
        Self {
            sale_id: sale.id,
            total_amount: sale.total_amount.value().to_f64().unwrap_or(0.0),
            payment_status: payment.status,
            payment_ref: payment.transaction_ref,
            receipt_code: Some(receipt.receipt_code.as_str().to_string()),
            receipt_status: Some(receipt.status),
            created_at: sale.created_at,
        }
    }
}

impl SaleResponse {
    /// Compose variant for optional payment/receipt lookups
    pub fn compose(
        sale: SaleEntity,
        payment: Option<PaymentEntity>,
        receipt: Option<ReceiptEntity>,
    ) -> Self {
        Self {
            sale_id: sale.id,
            total_amount: sale.total_amount.value().to_f64().unwrap_or(0.0),
            payment_status: payment
                .as_ref()
                .map(|p| p.status.clone())
                .unwrap_or_else(|| "UNKNOWN".into()),
            payment_ref: payment.and_then(|p| p.transaction_ref),
            receipt_code: receipt.as_ref().map(|r| r.receipt_code.as_str().to_string()),
            receipt_status: receipt.map(|r| r.status),
            created_at: sale.created_at,
        }
    }
}
