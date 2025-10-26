use crate::domain::value_objects::{isbn13::Isbn13, money::Money};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BookEntity {
    pub isbn: Isbn13,
    pub title: String,
    pub author: Option<String>,
    pub synopsis: Option<String>,
    pub price: Money,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
