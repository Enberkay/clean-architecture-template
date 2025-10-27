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

impl BookEntity {
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn update_price(&mut self, new_price: Money) {
        self.price = new_price;
        self.updated_at = Utc::now();
    }

    pub fn update_info(&mut self, title: String, author: Option<String>, synopsis: Option<String>) {
        self.title = title;
        self.author = author;
        self.synopsis = synopsis;
        self.updated_at = Utc::now();
    }

    pub fn is_available(&self) -> bool {
        self.is_active
    }
}
