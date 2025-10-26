use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BookEntity {
    pub isbn: i32,
    pub title: String,
    pub author: String,
    pub synopsis: String,
    pub price: f32,
    pub cover_image_url: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
