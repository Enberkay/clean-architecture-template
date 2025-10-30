use crate::domain::domain_errors::{DomainError, DomainResult};
use crate::domain::entities::permission::PermissionEntity;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RoleEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<PermissionEntity>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RoleEntity {
    /// Create a new role with validation
    pub fn new(name: String, description: Option<String>) -> DomainResult<Self> {
        Self::validate_name(&name)?;
        let now = Utc::now();

        Ok(Self {
            id: 0,
            name: name.trim().to_uppercase(),
            description: description.map(|d| d.trim().to_string()),
            permissions: Vec::new(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Rename this role
    pub fn rename(&mut self, new_name: String) -> DomainResult<()> {
        Self::validate_name(&new_name)?;
        self.name = new_name.trim().to_uppercase();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update description
    pub fn update_description(&mut self, desc: Option<String>) {
        self.description = desc.map(|d| d.trim().to_string());
        self.updated_at = Utc::now();
    }

    /// Assign permissions to role
    pub fn set_permissions(&mut self, permissions: Vec<PermissionEntity>) -> DomainResult<()> {
        self.permissions = permissions;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Domain rule: admin check
    pub fn is_admin(&self) -> bool {
        self.name.eq_ignore_ascii_case("ADMIN")
    }

    /// Validation for name
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

    pub fn validate(&self) -> DomainResult<()> {
        Self::validate_name(&self.name)
    }

    /// Summary for logging
    pub fn summary(&self) -> String {
        format!(
            "Role(id={}, name='{}', permissions={}, desc={})",
            self.id,
            self.name,
            self.permissions.len(),
            self.description
                .as_deref()
                .unwrap_or("No description")
        )
    }
}
