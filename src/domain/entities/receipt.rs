use crate::domain::{
    domain_errors::{DomainError, DomainResult},
    value_objects::{money::Money, receipt_code::ReceiptCode},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ReceiptEntity {
    pub id: i32,
    pub receipt_code: ReceiptCode,
    pub type_name: String,     // "SALE" | "ORDER"
    pub reference_id: i32,     // FK: sales.id or orders.id
    pub source: String,        // "POS" | "ONLINE"
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: Money,
    pub payment_method: Option<String>,
    pub payment_ref: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub status: String,        // "PAID" | "CANCELLED"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReceiptEntity {
    /// Create a new receipt (must validate type, source, and total_amount)
    pub fn new(
        receipt_code: ReceiptCode,
        type_name: String,
        reference_id: i32,
        source: String,
        total_amount: Money,
        user_id: Option<i32>,
        branch_id: Option<i32>,
        payment_method: Option<String>,
    ) -> DomainResult<Self> {
        Self::validate_type(&type_name)?;
        Self::validate_source(&source)?;
        if total_amount.value() <= 0.0 {
            return Err(DomainError::validation("Total amount must be greater than zero"));
        }

        let now = Utc::now();
        Ok(Self {
            id: 0,
            receipt_code,
            type_name: type_name.to_uppercase(),
            reference_id,
            source: source.to_uppercase(),
            user_id,
            branch_id,
            total_amount,
            payment_method: payment_method.map(|p| p.to_uppercase()),
            payment_ref: None,
            issued_at: now,
            status: "PAID".into(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Mark receipt as cancelled
    pub fn mark_cancelled(&mut self) -> DomainResult<()> {
        if self.status == "CANCELLED" {
            return Err(DomainError::validation("Receipt is already cancelled"));
        }
        self.status = "CANCELLED".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark receipt as paid (e.g. after successful transaction)
    pub fn mark_paid(&mut self) -> DomainResult<()> {
        if self.status == "PAID" {
            return Err(DomainError::validation("Receipt is already marked as paid"));
        }
        self.status = "PAID".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update payment reference (optional)
    pub fn set_payment_ref(&mut self, ref_code: Option<String>) {
        self.payment_ref = ref_code;
        self.updated_at = Utc::now();
    }

    /// Simple string summary
    pub fn summary(&self) -> String {
        format!(
            "{} [{}] - {} - {:.2} {}",
            self.receipt_code.as_str(),
            self.type_name,
            self.status,
            self.total_amount.value(),
            self.payment_method.clone().unwrap_or_else(|| "N/A".into())
        )
    }

    // -----------------------------------------
    // Validation helpers (business constraints)
    // -----------------------------------------

    fn validate_type(type_name: &str) -> DomainResult<()> {
        let valid_types = ["SALE", "ORDER"];
        if !valid_types.contains(&type_name.to_uppercase().as_str()) {
            return Err(DomainError::validation(format!(
                "Invalid receipt type: {}",
                type_name
            )));
        }
        Ok(())
    }

    fn validate_source(source: &str) -> DomainResult<()> {
        let valid_sources = ["POS", "ONLINE"];
        if !valid_sources.contains(&source.to_uppercase().as_str()) {
            return Err(DomainError::validation(format!(
                "Invalid receipt source: {}",
                source
            )));
        }
        Ok(())
    }

    /// Validate integrity of current entity
    pub fn validate(&self) -> DomainResult<()> {
        Self::validate_type(&self.type_name)?;
        Self::validate_source(&self.source)?;
        if self.total_amount.value() <= 0.0 {
            return Err(DomainError::validation("Total amount must be greater than zero"));
        }
        Ok(())
    }
}
