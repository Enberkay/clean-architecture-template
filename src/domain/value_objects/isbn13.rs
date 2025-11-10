use anyhow::{Result, anyhow};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Isbn13(String);

impl Isbn13 {
    pub fn new(value: &str) -> Result<Self> {
        let trimmed = value.trim().replace("-", "");
        if trimmed.len() != 13 || !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Err(anyhow!(
                "ISBN-13 must contain exactly 13 digits",
            ));
        }

        if !Self::is_valid_checksum(&trimmed) {
            return Err(anyhow!("Invalid ISBN-13 checksum"));
        }

        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid_checksum(value: &str) -> bool {
        let digits: Vec<u32> = value.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 13 {
            return false;
        }

        let sum: u32 = digits[..12]
            .iter()
            .enumerate()
            .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
            .sum();

        let checksum = (10 - (sum % 10)) % 10;
        checksum == digits[12]
    }
}

impl fmt::Display for Isbn13 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
