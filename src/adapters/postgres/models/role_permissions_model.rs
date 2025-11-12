use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ======================
// RolePermissionModel (SQLx)
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RolePermissionModel {
    pub role_id: i32,
    pub permission_id: i32,
    pub assigned_at: DateTime<Utc>,
}
