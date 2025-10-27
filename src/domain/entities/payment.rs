use crate::domain::{
    domain_errors::{DomainError, DomainResult},
    value_objects::money::Money,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PaymentEntity {
    pub id: i32,
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub payment_method: String,   // "CASH", "CREDIT_CARD", "PROMPTPAY", etc.
    pub transaction_ref: Option<String>,
    pub amount: Money,
    pub status: String,           // "PENDING", "PAID", "FAILED", "REFUNDED"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PaymentEntity {
    /// Create a new payment in `PENDING` state
    pub fn new(
        order_id: Option<i32>,
        sale_id: Option<i32>,
        payment_method: String,
        amount: Money,
    ) -> DomainResult<Self> {
        Self::validate_method(&payment_method)?;
        if amount.value() <= 0.0 {
            return Err(DomainError::validation("Payment amount must be greater than zero"));
        }

        let now = Utc::now();
        Ok(Self {
            id: 0,
            order_id,
            sale_id,
            payment_method: payment_method.to_uppercase(),
            transaction_ref: None,
            amount,
            status: "PENDING".into(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Mark this payment as successfully completed
    pub fn mark_paid(&mut self, transaction_ref: Option<String>) -> DomainResult<()> {
        if self.status != "PENDING" {
            return Err(DomainError::validation("Only pending payments can be marked as paid"));
        }

        self.status = "PAID".into();
        self.transaction_ref = transaction_ref;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark this payment as failed
    pub fn mark_failed(&mut self) -> DomainResult<()> {
        if self.status == "PAID" {
            return Err(DomainError::validation("Cannot fail a payment that is already paid"));
        }

        self.status = "FAILED".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark this payment as refunded
    pub fn mark_refunded(&mut self) -> DomainResult<()> {
        if self.status != "PAID" {
            return Err(DomainError::validation("Only paid payments can be refunded"));
        }

        self.status = "REFUNDED".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Validate payment data invariants
    pub fn validate(&self) -> DomainResult<()> {
        Self::validate_method(&self.payment_method)?;
        if self.amount.value() <= 0.0 {
            return Err(DomainError::validation("Payment amount must be greater than zero"));
        }
        Ok(())
    }

    /// Return true if payment is settled
    pub fn is_paid(&self) -> bool {
        self.status.eq_ignore_ascii_case("PAID")
    }

    /// Return true if payment failed
    pub fn is_failed(&self) -> bool {
        self.status.eq_ignore_ascii_case("FAILED")
    }

    /// Return true if payment is refunded
    pub fn is_refunded(&self) -> bool {
        self.status.eq_ignore_ascii_case("REFUNDED")
    }

    /// Display a short summary
    pub fn summary(&self) -> String {
        format!(
            "[{}] {} {:.2} ({})",
            self.status,
            self.payment_method,
            self.amount.value(),
            self.transaction_ref
                .clone()
                .unwrap_or_else(|| "no-ref".into())
        )
    }

    // ---------------------------
    // Internal validation helpers
    // ---------------------------
    fn validate_method(method: &str) -> DomainResult<()> {
        let valid_methods = ["CASH", "CREDIT_CARD", "PROMPTPAY", "BANK_TRANSFER"];
        if !valid_methods.contains(&method.to_uppercase().as_str()) {
            return Err(DomainError::validation(format!("Invalid payment method: {}", method)));
        }
        Ok(())
    }
}
