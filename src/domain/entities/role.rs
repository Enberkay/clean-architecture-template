use crate::domain::domain_errors::{DomainError, DomainResult};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RoleEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl RoleEntity {
    /// Create a new role with validation
    pub fn new(name: String, description: Option<String>) -> DomainResult<Self> {
        Self::validate_name(&name)?;
        Ok(Self {
            id: 0,
            name: name.trim().to_uppercase(),
            description: description.map(|d| d.trim().to_string()),
            created_at: Utc::now(),
        })
    }

    /// Rename this role
    pub fn rename(&mut self, new_name: String) -> DomainResult<()> {
        Self::validate_name(&new_name)?;
        self.name = new_name.trim().to_uppercase();
        Ok(())
    }

    /// Update description
    pub fn update_description(&mut self, desc: Option<String>) {
        self.description = desc.map(|d| d.trim().to_string());
    }

    /// Check if role is administrative (domain logic)
    pub fn is_admin(&self) -> bool {
        self.name.eq_ignore_ascii_case("ADMIN")
    }

    /// Return a summary string for logging / display
    pub fn summary(&self) -> String {
        format!(
            "Role(id={}, name='{}', desc={})",
            self.id,
            self.name,
            self.description
                .as_deref()
                .unwrap_or("No description")
        )
    }

    /// Validate role name
    fn validate_name(name: &str) -> DomainResult<()> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::validation("Role name cannot be empty"));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::validation("Role name too long (max 100 chars)"));
        }
        Ok(())
    }
}
