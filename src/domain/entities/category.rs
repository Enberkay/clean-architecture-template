use crate::domain::domain_errors::{DomainError, DomainResult};
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
    pub fn new(name: String, description: Option<String>) -> DomainResult<Self> {
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
    pub fn rename(&mut self, new_name: String) -> DomainResult<()> {
        Self::validate_name(&new_name)?;
        self.name = new_name;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update description (empty string not allowed)
    pub fn update_description(&mut self, desc: Option<String>) -> DomainResult<()> {
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

    fn validate_name(name: &str) -> DomainResult<()> {
        if name.trim().is_empty() {
            return Err(DomainError::validation("Category name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(DomainError::validation("Category name is too long (max 100 chars)"));
        }
        Ok(())
    }

    fn validate_description(desc: &str) -> DomainResult<()> {
        if desc.trim().is_empty() {
            return Err(DomainError::validation("Description cannot be empty"));
        }
        Ok(())
    }
}
