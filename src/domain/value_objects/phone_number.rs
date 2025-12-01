use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone: String) -> Result<Self> {
        let trimmed = phone.trim();
        if trimmed.len() < 6 || trimmed.len() > 20 {
            return Err(anyhow!("Phone number length invalid"));
        }
        if !trimmed.chars().all(|c| c.is_ascii_digit() || c == '+' || c == '-') {
            return Err(anyhow!("Phone number must contain only digits, '+', or '-'"));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}