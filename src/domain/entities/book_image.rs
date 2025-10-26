use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BookImageEntity {
    pub id: i32,
    pub book_isbn: String,
    pub image_url: String,
    pub image_type: String, // COVER, PREVIEW, GALLERY
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}
