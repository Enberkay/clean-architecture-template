use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CategoryEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryEntity {
    /// Create a new category with validation.
    pub fn new(name: String, description: Option<String>) -> Result<Self> {
        Self::validate_name(&name)?;
        if let Some(desc) = &description {
            Self::validate_description(desc)?;
        }

        let now = Utc::now();
        Ok(Self {
            id: 0,
            name,
            description,
            created_at: now,
            updated_at: now,
        })
    }

    /// Rename the category (validated)
    pub fn rename(&mut self, new_name: String) -> Result<()> {
        Self::validate_name(&new_name)?;
        self.name = new_name;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update description (empty string not allowed)
    pub fn update_description(&mut self, desc: Option<String>) -> Result<()> {
        if let Some(d) = &desc {
            Self::validate_description(d)?;
        }
        self.description = desc;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Checks if category has a description
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // ==========================
    // Internal validators
    // ==========================

    fn validate_name(name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(anyhow!("Category name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(anyhow!("Category name is too long (max 100 chars)"));
        }
        Ok(())
    }

    fn validate_description(desc: &str) -> Result<()> {
        if desc.trim().is_empty() {
            return Err(anyhow!("Description cannot be empty"));
        }
        Ok(())
    }
}
