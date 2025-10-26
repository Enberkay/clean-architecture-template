use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BranchEntity {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}
