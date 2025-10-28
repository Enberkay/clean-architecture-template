use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use bigdecimal::{BigDecimal, ToPrimitive, FromPrimitive};

use crate::{
    infrastructure::postgres::schema::{orders, order_items},
    domain::{
        entities::order::{OrderEntity, OrderItemEntity},
        value_objects::money::Money,
    },
};

// ======================
// OrderModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = orders)]
#[diesel(primary_key(id))]
pub struct OrderModel {
    pub id: i32,
    pub user_id: Option<i32>,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub source: String,
    pub total_amount: BigDecimal,
    pub shipping_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<OrderModel> for OrderEntity {
    fn from(model: OrderModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            order_date: model.order_date,
            status: model.status,
            source: model.source,
            total_amount: Money::new(model.total_amount.to_f64().unwrap_or(0.0))
                .expect("Invalid total amount"),
            shipping_address: model.shipping_address,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<OrderEntity> for OrderModel {
    fn from(entity: OrderEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            order_date: entity.order_date,
            status: entity.status,
            source: entity.source,
            total_amount: BigDecimal::from_f64(entity.total_amount.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            shipping_address: entity.shipping_address,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

// ======================
// OrderItemModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = order_items)]
#[diesel(primary_key(id))]
pub struct OrderItemModel {
    pub id: i32,
    pub order_id: i32,
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_purchase: BigDecimal,
    pub subtotal: BigDecimal,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<OrderItemModel> for OrderItemEntity {
    fn from(model: OrderItemModel) -> Self {
        Self {
            id: model.id,
            order_id: model.order_id,
            book_isbn: model.book_isbn,
            book_title: model.book_title,
            book_author: model.book_author,
            quantity: model.quantity,
            price_at_purchase: Money::new(model.price_at_purchase.to_f64().unwrap_or(0.0))
                .expect("Invalid price"),
            subtotal: Money::new(model.subtotal.to_f64().unwrap_or(0.0))
                .expect("Invalid subtotal"),
            created_at: model.created_at,
        }
    }
}

impl From<OrderItemEntity> for OrderItemModel {
    fn from(entity: OrderItemEntity) -> Self {
        Self {
            id: entity.id,
            order_id: entity.order_id,
            book_isbn: entity.book_isbn,
            book_title: entity.book_title,
            book_author: entity.book_author,
            quantity: entity.quantity,
            price_at_purchase: BigDecimal::from_f64(entity.price_at_purchase.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            subtotal: BigDecimal::from_f64(entity.subtotal.value())
                .unwrap_or_else(|| BigDecimal::from(0)),
            created_at: entity.created_at,
        }
    }
}
