use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::entities::permission::PermissionEntity;

#[derive(Debug, Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PermissionEntity> for PermissionResponse {
    fn from(entity: PermissionEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
