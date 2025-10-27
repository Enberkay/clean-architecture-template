use crate::domain::domain_errors::{DomainError, DomainResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Isbn13(String);

impl Isbn13 {
    /// Creates a new Isbn13 after validating its format and checksum.
    pub fn new(value: &str) -> DomainResult<Self> {
        let trimmed = value.trim().replace("-", ""); // allow with or without '-'

        // 1. Must be 13 digits
        if trimmed.len() != 13 || !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Err(DomainError::validation(
                "ISBN-13 must contain exactly 13 digits",
            ));
        }

        // 2. Validate checksum
        if !Self::is_valid_checksum(&trimmed) {
            return Err(DomainError::validation("Invalid ISBN-13 checksum"));
        }

        Ok(Self(trimmed))
    }

    /// Returns the internal string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compute and verify ISBN-13 checksum
    fn is_valid_checksum(value: &str) -> bool {
        let digits: Vec<u32> = value.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 13 {
            return false;
        }

        // checksum: (sum of digits at odd positions) + (3 * sum of digits at even positions)
        // The total mod 10 must be 0
        let sum: u32 = digits[..12]
            .iter()
            .enumerate()
            .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
            .sum();

        let checksum = (10 - (sum % 10)) % 10;
        checksum == digits[12]
    }
}
