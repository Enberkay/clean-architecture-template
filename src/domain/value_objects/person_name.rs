use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonName(String);

impl PersonName {
    pub fn new(name: String) -> Result<Self> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Name cannot be empty"));
        }
        if trimmed.len() > 100 {
            return Err(anyhow!("Name too long (max 100 chars)"));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}