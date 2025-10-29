use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

use crate::domain::{
    entities::payment::PaymentEntity,
    value_objects::money::Money,
};

// ======================
// PaymentModel (SQLx Model)
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PaymentModel {
    pub id: i32,
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub payment_method: String,
    pub transaction_ref: Option<String>,
    pub amount: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<PaymentModel> for PaymentEntity {
    fn from(model: PaymentModel) -> Self {
        Self {
            id: model.id,
            order_id: model.order_id,
            sale_id: model.sale_id,
            payment_method: model.payment_method,
            transaction_ref: model.transaction_ref,
            amount: Money::from_decimal(model.amount)
                .expect("Invalid payment amount"),
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<PaymentEntity> for PaymentModel {
    fn from(entity: PaymentEntity) -> Self {
        Self {
            id: entity.id,
            order_id: entity.order_id,
            sale_id: entity.sale_id,
            payment_method: entity.payment_method,
            transaction_ref: entity.transaction_ref,
            amount: entity.amount.to_decimal(),
            status: entity.status,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
