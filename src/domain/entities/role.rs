use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::{
    role_name::RoleName,
    role_description::RoleDescription,
};

#[derive(Debug, Clone)]
pub struct RoleEntity {
    pub id: i32,
    pub name: RoleName,
    pub description: Option<RoleDescription>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RoleEntity {
    pub fn new(name: String, description: Option<String>) -> Result<Self> {
        let name_vo = RoleName::new(name)?;
        
        let desc_vo = match description {
            Some(d) => Some(RoleDescription::new(d)?),
            None => None,
        };

        let now = Utc::now();

        Ok(Self {
            id: 0,
            name: name_vo,
            description: desc_vo,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn rename(&mut self, new_name: String) -> Result<()> {
        self.name = RoleName::new(new_name)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_description(&mut self, desc: Option<String>) -> Result<()> {
        self.description = match desc {
            Some(d) => Some(RoleDescription::new(d)?),
            None => None,
        };
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_admin(&self) -> bool {
        self.name.as_str() == "ADMIN"
    }

    pub fn summary(&self) -> String {
        format!(
            "Role(id={}, name='{}', desc={})",
            self.id,
            self.name.as_str(),
            self.description
                .as_ref()
                .map(|d| d.as_str())
                .unwrap_or("No description")
        )
    }
}