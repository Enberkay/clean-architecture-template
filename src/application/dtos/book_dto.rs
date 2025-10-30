use serde::{Serialize, Deserialize};
use rust_decimal::prelude::ToPrimitive;
use crate::domain::entities::book::BookEntity;

#[derive(Debug, Deserialize)]
pub struct CreateBookRequest {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub synopsis: String,
    pub price: f64,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookRequest {
    pub title: Option<String>,
    pub author: Option<String>,
    pub synopsis: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BookResponse {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub synopsis: String,
    pub price: f64,
    pub is_active: bool,
}

impl From<BookEntity> for BookResponse {
    fn from(book: BookEntity) -> Self {
        Self {
            isbn: book.isbn.to_string(),
            title: book.title,
            author: book.author.unwrap_or_default(),
            synopsis: book.synopsis.unwrap_or_default(),
            price: book.price.value().to_f64().unwrap_or(0.0),
            is_active: book.is_active,
        }
    }
}
