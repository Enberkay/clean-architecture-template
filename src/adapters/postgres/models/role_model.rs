use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::entities::role::RoleEntity;

// ======================
// RoleModel (SQLx)
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoleModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

// RolePermissionModel for JOIN queries
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RolePermissionModel {
    pub role_id: i32,
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<RoleModel> for RoleEntity {
    fn from(model: RoleModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            permissions: Vec::new(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<RoleEntity> for RoleModel {
    fn from(entity: RoleEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
