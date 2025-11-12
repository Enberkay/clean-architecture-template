use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::entities::{role::RoleEntity, permission::PermissionEntity};

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
    pub permissions: Vec<PermissionSummary>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PermissionSummary {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<RoleEntity> for RoleResponse {
    fn from(role: RoleEntity) -> Self {
        Self {
            id: role.id,
            name: role.name.clone(),
            description: role.description.clone(),
            permissions: role
                .permissions
                .into_iter()
                .map(PermissionSummary::from)
                .collect(),
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl From<PermissionEntity> for PermissionSummary {
    fn from(p: PermissionEntity) -> Self {
        Self {
            id: p.id,
            name: p.name,
            description: p.description,
        }
    }
}
