use anyhow::{Result, anyhow};
use crate::domain::value_objects::{isbn13::Isbn13, money::Money};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

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
    /// Creates a new book entity.
    pub fn new(
        isbn: Isbn13,
        title: String,
        author: Option<String>,
        synopsis: Option<String>,
        price: Money,
    ) -> Result<Self> {
        if title.trim().is_empty() {
            return Err(anyhow!("Book title cannot be empty"));
        }

        if price.value() <= Decimal::ZERO {
            return Err(anyhow!("Book price must be greater than zero"));
        }

        let now = Utc::now();
        Ok(Self {
            isbn,
            title,
            author,
            synopsis,
            price,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Update book price with validation.
    pub fn update_price(&mut self, new_price: Money) -> Result<()> {
        if new_price.value() <= Decimal::ZERO {
            return Err(anyhow!("Book price must be greater than zero"));
        }
        self.price = new_price;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update general book info.
    pub fn update_info(
        &mut self,
        title: String,
        author: Option<String>,
        synopsis: Option<String>,
    ) -> Result<()> {
        if title.trim().is_empty() {
            return Err(anyhow!("Book title cannot be empty"));
        }

        self.title = title;
        self.author = author;
        self.synopsis = synopsis;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if this book is currently available for sale.
    pub fn is_available(&self) -> bool {
        self.is_active
    }
}
