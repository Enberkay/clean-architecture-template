use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CategoryEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
