use crate::domain::{
    domain_errors::{DomainError, DomainResult},
    value_objects::money::Money,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct OrderEntity {
    pub id: i32,
    pub user_id: Option<i32>,
    pub order_date: DateTime<Utc>,
    pub status: String, // "PENDING" | "PAID" | "SHIPPED" | "CANCELLED"
    pub source: String, // "ONLINE" | "POS"
    pub total_amount: Money,
    pub shipping_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OrderEntity {
    pub fn new(user_id: Option<i32>, source: String, shipping_address: Option<String>) -> DomainResult<Self> {
        Self::validate_source(&source)?;

        let now = Utc::now();
        Ok(Self {
            id: 0,
            user_id,
            order_date: now,
            status: "PENDING".to_string(),
            source: source.to_uppercase(),
            total_amount: Money::zero(),
            shipping_address,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn mark_paid(&mut self) -> DomainResult<()> {
        if self.status != "PENDING" {
            return Err(DomainError::validation("Only pending orders can be marked as paid"));
        }
        self.status = "PAID".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn mark_shipped(&mut self) -> DomainResult<()> {
        if self.status != "PAID" {
            return Err(DomainError::validation("Only paid orders can be shipped"));
        }
        self.status = "SHIPPED".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn cancel(&mut self) -> DomainResult<()> {
        if self.status == "SHIPPED" {
            return Err(DomainError::validation("Cannot cancel an order that has already been shipped"));
        }
        if self.status == "CANCELLED" {
            return Err(DomainError::validation("Order is already cancelled"));
        }
        self.status = "CANCELLED".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_total(&mut self, new_total: Money) -> DomainResult<()> {
        if new_total.value() < 0.0 {
            return Err(DomainError::validation("Total amount cannot be negative"));
        }
        self.total_amount = new_total;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn can_ship(&self) -> bool {
        self.status == "PAID"
    }

    pub fn can_cancel(&self) -> bool {
        self.status == "PENDING"
    }

    pub fn is_paid(&self) -> bool {
        self.status == "PAID"
    }

    fn validate_source(source: &str) -> DomainResult<()> {
        let valid_sources = ["ONLINE", "POS"];
        if !valid_sources.contains(&source.to_uppercase().as_str()) {
            return Err(DomainError::validation(format!("Invalid order source: {}", source)));
        }
        Ok(())
    }
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

impl OrderItemEntity {
    pub fn new(
        order_id: i32,
        book_isbn: String,
        book_title: String,
        book_author: Option<String>,
        quantity: i32,
        price_at_purchase: Money,
    ) -> DomainResult<Self> {
        if quantity <= 0 {
            return Err(DomainError::validation("Quantity must be greater than zero"));
        }

        let subtotal = price_at_purchase.multiply(quantity as u32);
        Ok(Self {
            id: 0,
            order_id,
            book_isbn,
            book_title,
            book_author,
            quantity,
            price_at_purchase,
            subtotal,
            created_at: Utc::now(),
        })
    }

    pub fn update_quantity(&mut self, qty: i32) -> DomainResult<()> {
        if qty <= 0 {
            return Err(DomainError::validation("Quantity must be greater than zero"));
        }
        self.quantity = qty;
        self.subtotal = self.price_at_purchase.multiply(qty as u32);
        Ok(())
    }

    pub fn recalculate_subtotal(&mut self) {
        self.subtotal = self.price_at_purchase.multiply(self.quantity as u32);
    }

    pub fn subtotal_value(&self) -> f64 {
        self.subtotal.value()
    }
}
