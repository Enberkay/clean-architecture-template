use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BookEntity {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub synopsis: String,
    pub price: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
