use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleDescription(String);

impl RoleDescription {
    pub fn new(description: String) -> Result<Self> {
        let trimmed = description.trim();
        if trimmed.len() > 255 {
            return Err(anyhow!("Description too long (max 255 chars)"));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}