use crate::domain::domain_errors::{DomainError, DomainResult};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BranchEntity {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BranchEntity {
    /// Create a new branch with validation.
    pub fn new(name: String, address: Option<String>, phone: Option<String>) -> DomainResult<Self> {
        Self::validate_name(&name)?;
        if let Some(p) = &phone {
            Self::validate_phone(p)?;
        }

        let now = Utc::now();
        Ok(Self {
            id: 0,
            name,
            address,
            phone,
            created_at: now,
            updated_at: now,
        })
    }

    /// Update branch information with validation.
    pub fn update_info(
        &mut self,
        name: String,
        address: Option<String>,
        phone: Option<String>,
    ) -> DomainResult<()> {
        Self::validate_name(&name)?;
        if let Some(p) = &phone {
            Self::validate_phone(p)?;
        }

        self.name = name;
        self.address = address;
        self.phone = phone;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if branch has a phone number
    pub fn has_phone(&self) -> bool {
        self.phone.is_some()
    }

    /// Check if branch has an address
    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // ==========================
    // Internal validators
    // ==========================

    fn validate_name(name: &str) -> DomainResult<()> {
        if name.trim().is_empty() {
            return Err(DomainError::validation("Branch name cannot be empty"));
        }
        Ok(())
    }

    fn validate_phone(phone: &str) -> DomainResult<()> {
        if !phone.chars().all(|c| c.is_ascii_digit() || c == '+' || c == '-' || c == ' ') {
            return Err(DomainError::validation("Phone number contains invalid characters"));
        }
        if phone.trim().len() < 6 {
            return Err(DomainError::validation("Phone number is too short"));
        }
        Ok(())
    }
}
