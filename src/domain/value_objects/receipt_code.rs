use crate::domain::domain_errors::{DomainError, DomainResult};
use chrono::{Datelike, Utc};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReceiptCode(String);

impl ReceiptCode {
    /// Create a new validated receipt code.
    pub fn new(value: &str) -> DomainResult<Self> {
        let trimmed = value.trim();

        if !Self::is_valid_format(trimmed) {
            return Err(DomainError::validation(format!(
                "Invalid receipt code format: {}",
                trimmed
            )));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Generate a new ReceiptCode automatically.
    /// Example: "POS-2025-00042"
    pub fn generate(prefix: &str, sequence: u32) -> DomainResult<Self> {
        let year = Utc::now().year();
        let code = format!("{}-{}-{:05}", prefix.to_uppercase(), year, sequence);
        Self::new(&code)
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate format: PREFIX-YYYY-NNNNN
    fn is_valid_format(value: &str) -> bool {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        let (prefix, year, seq) = (parts[0], parts[1], parts[2]);

        prefix.chars().all(|c| c.is_ascii_uppercase())
            && year.len() == 4
            && year.chars().all(|c| c.is_ascii_digit())
            && seq.len() == 5
            && seq.chars().all(|c| c.is_ascii_digit())
    }
}

impl fmt::Display for ReceiptCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
