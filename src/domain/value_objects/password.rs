use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self> {
        if password.trim().len() < 8 {
            return Err(anyhow!("Password must be at least 8 characters long"));
        }
        Ok(Self(password))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}