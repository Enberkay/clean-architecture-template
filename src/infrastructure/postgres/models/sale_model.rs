use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use bigdecimal::{BigDecimal, ToPrimitive, FromPrimitive};

use crate::{
    infrastructure::postgres::schema::{sales, sale_items},
    domain::{
        entities::sale::{SaleEntity, SaleItemEntity},
        value_objects::money::Money,
    },
};

// ======================
// SaleModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = sales)]
#[diesel(primary_key(id))]
pub struct SaleModel {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub sale_date: DateTime<Utc>,
    pub total_amount: BigDecimal,
    pub payment_method: String,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<SaleModel> for SaleEntity {
    fn from(model: SaleModel) -> Self {
        Self {
            id: model.id,
            employee_id: model.employee_id,
            branch_id: model.branch_id,
            sale_date: model.sale_date,
            total_amount: Money::new(model.total_amount.to_f64().unwrap_or(0.0))
                .expect("Invalid total amount"),
            payment_method: model.payment_method,
            created_at: model.created_at,
        }
    }
}

impl From<SaleEntity> for SaleModel {
    fn from(entity: SaleEntity) -> Self {
        Self {
            id: entity.id,
            employee_id: entity.employee_id,
            branch_id: entity.branch_id,
            sale_date: entity.sale_date,
            total_amount: BigDecimal::from_f64(entity.total_amount.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            payment_method: entity.payment_method,
            created_at: entity.created_at,
        }
    }
}

// ======================
// SaleItemModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = sale_items)]
#[diesel(primary_key(id))]
pub struct SaleItemModel {
    pub id: i32,
    pub sale_id: i32,
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_sale: BigDecimal,
    pub subtotal: BigDecimal,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<SaleItemModel> for SaleItemEntity {
    fn from(model: SaleItemModel) -> Self {
        Self {
            id: model.id,
            sale_id: model.sale_id,
            book_isbn: model.book_isbn,
            book_title: model.book_title,
            book_author: model.book_author,
            quantity: model.quantity,
            price_at_sale: Money::new(model.price_at_sale.to_f64().unwrap_or(0.0))
                .expect("Invalid price"),
            subtotal: Money::new(model.subtotal.to_f64().unwrap_or(0.0))
                .expect("Invalid subtotal"),
            created_at: model.created_at,
        }
    }
}

impl From<SaleItemEntity> for SaleItemModel {
    fn from(entity: SaleItemEntity) -> Self {
        Self {
            id: entity.id,
            sale_id: entity.sale_id,
            book_isbn: entity.book_isbn,
            book_title: entity.book_title,
            book_author: entity.book_author,
            quantity: entity.quantity,
            price_at_sale: BigDecimal::from_f64(entity.price_at_sale.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            subtotal: BigDecimal::from_f64(entity.subtotal.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            created_at: entity.created_at,
        }
    }
}
