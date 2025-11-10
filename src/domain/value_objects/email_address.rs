use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(value: &str) -> Result<Self> {
        let trimmed = value.trim();

        // Must contain '@'
        let at_index = trimmed.find('@').ok_or_else(|| {
            anyhow!("Email must contain '@'")
        })?;

        // Must contain '.' after '@'
        let domain_part = &trimmed[at_index + 1..];
        if !domain_part.contains('.') {
            return Err(anyhow!("Email domain must contain '.'"));
        }

        // Must have reasonable length
        if trimmed.len() < 5 {
            return Err(anyhow!("Email too short"));
        }

        Ok(Self(trimmed.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
