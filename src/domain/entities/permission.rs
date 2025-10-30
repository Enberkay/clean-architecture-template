use crate::domain::domain_errors::{DomainError, DomainResult};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PermissionEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PermissionEntity {
    /// Create a new permission entity with validation
    pub fn new(name: String, description: Option<String>) -> DomainResult<Self> {
        Self::validate_name(&name)?;
        Ok(Self {
            id: 0,
            name: name.trim().to_string(),
            description: description.map(|d| d.trim().to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Rename permission (must not be empty or too long)
    pub fn rename(&mut self, new_name: String) -> DomainResult<()> {
        Self::validate_name(&new_name)?;
        self.name = new_name.trim().to_string();
        Ok(())
    }

    /// Update description (optional)
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description.map(|d| d.trim().to_string());
    }

    /// Validate the entity fields (for service layer)
    pub fn validate(&self) -> DomainResult<()> {
        Self::validate_name(&self.name)?;
        Ok(())
    }

    /// Validate permission name (business rule)
    fn validate_name(name: &str) -> DomainResult<()> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::validation("Permission name cannot be empty"));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::validation("Permission name too long (max 100 chars)"));
        }
        Ok(())
    }

    /// Return a display summary
    pub fn summary(&self) -> String {
        format!(
            "Permission(id={}, name='{}', desc={})",
            self.id,
            self.name,
            self.description.as_deref().unwrap_or("No description")
        )
    }
}
