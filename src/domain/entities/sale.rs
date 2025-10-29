use crate::domain::{
    domain_errors::{DomainError, DomainResult},
    value_objects::money::Money,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

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
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_sale: Money,
    pub subtotal: Money,
    pub created_at: DateTime<Utc>,
}

impl SaleEntity {
    /// Create new Sale record
    pub fn new(
        employee_id: Option<i32>,
        branch_id: Option<i32>,
        payment_method: String,
    ) -> DomainResult<Self> {
        Self::validate_payment_method(&payment_method)?;
        let now = Utc::now();
        Ok(Self {
            id: 0,
            employee_id,
            branch_id,
            sale_date: now,
            total_amount: Money::zero(),
            payment_method: payment_method.trim().to_uppercase(),
            created_at: now,
        })
    }

    /// Update total amount (must not be negative)
    pub fn update_total(&mut self, new_total: Money) -> DomainResult<()> {
        if new_total.value() < Decimal::ZERO {
            return Err(DomainError::validation("Total amount cannot be negative"));
        }
        self.total_amount = new_total;
        Ok(())
    }

    /// Add item subtotal to total
    pub fn add_item(&mut self, item_total: Money) {
        self.total_amount = self.total_amount.add(item_total);
    }

    /// Recalculate total from items
    pub fn calculate_total(&mut self, items: &[SaleItemEntity]) -> DomainResult<()> {
        let mut sum = Money::zero();
        for item in items {
            sum = sum.add(item.subtotal);
        }
        self.update_total(sum)
    }

    /// Validate all sale details before persisting
    pub fn validate(&self) -> DomainResult<()> {
        if self.payment_method.trim().is_empty() {
            return Err(DomainError::validation("Payment method cannot be empty"));
        }
        if self.total_amount.value() < Decimal::ZERO {
            return Err(DomainError::validation("Invalid total amount"));
        }
        Ok(())
    }

    fn validate_payment_method(method: &str) -> DomainResult<()> {
        let valid = ["CASH", "CARD", "PROMPTPAY", "TRANSFER"];
        let upper = method.trim().to_uppercase();
        if !valid.contains(&upper.as_str()) {
            return Err(DomainError::validation(format!(
                "Invalid payment method: {}",
                method
            )));
        }
        Ok(())
    }

    /// Quick summary string
    pub fn summary(&self) -> String {
        format!(
            "Sale(id={}, total={}, payment={}, date={})",
            self.id,
            self.total_amount.value(),
            self.payment_method,
            self.sale_date
        )
    }
}

impl SaleItemEntity {
    /// Create new SaleItem with validation
    pub fn new(
        sale_id: i32,
        book_isbn: String,
        book_title: String,
        book_author: Option<String>,
        quantity: i32,
        price_at_sale: Money,
    ) -> DomainResult<Self> {
        Self::validate_quantity(quantity)?;
        let subtotal = price_at_sale.multiply(quantity as u32);

        Ok(Self {
            id: 0,
            sale_id,
            book_isbn: book_isbn.trim().to_string(),
            book_title: book_title.trim().to_string(),
            book_author: book_author.map(|a| a.trim().to_string()),
            quantity,
            price_at_sale,
            subtotal,
            created_at: Utc::now(),
        })
    }

    /// Update quantity (must be > 0)
    pub fn update_quantity(&mut self, new_qty: i32) -> DomainResult<()> {
        Self::validate_quantity(new_qty)?;
        self.quantity = new_qty;
        self.subtotal = self.price_at_sale.multiply(new_qty as u32);
        Ok(())
    }

    fn validate_quantity(qty: i32) -> DomainResult<()> {
        if qty <= 0 {
            return Err(DomainError::validation("Quantity must be greater than zero"));
        }
        Ok(())
    }

    /// Validate this item
    pub fn validate(&self) -> DomainResult<()> {
        Self::validate_quantity(self.quantity)?;
        if self.subtotal.value() <= Decimal::ZERO {
            return Err(DomainError::validation("Subtotal must be greater than zero"));
        }
        Ok(())
    }

    /// Quick display summary
    pub fn summary(&self) -> String {
        format!(
            "{} x{} = {}",
            self.book_title,
            self.quantity,
            self.subtotal.value()
        )
    }
}
