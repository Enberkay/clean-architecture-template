use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RoleEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
