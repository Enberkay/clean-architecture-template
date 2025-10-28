use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable, Identifiable};

use crate::infrastructure::postgres::schema::role_permissions;

// ======================
// RolePermissionModel
// ======================

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = role_permissions)]
#[diesel(primary_key(role_id, permission_id))]
pub struct RolePermissionModel {
    pub role_id: i32,
    pub permission_id: i32,
    pub assigned_at: DateTime<Utc>,
}
