use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(name: String) -> Result<Self> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Role name cannot be empty"));
        }
        if trimmed.len() > 100 {
            return Err(anyhow!("Role name too long (max 100 chars)"));
        }
        Ok(Self(trimmed.to_uppercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RoleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}