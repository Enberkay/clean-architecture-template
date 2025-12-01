use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::entities::role::RoleEntity;

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub permission_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub permission_ids: Option<Vec<i32>>,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<RoleEntity> for RoleResponse {
    fn from(role: RoleEntity) -> Self {
        Self {
            id: role.id,
            name: role.name.clone(),
            description: role.description.clone(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}