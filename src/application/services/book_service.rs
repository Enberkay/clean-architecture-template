use std::sync::Arc;
use anyhow::Result;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

use crate::application::dtos::book_dto::{
    BookResponse,
    CreateBookRequest,
    UpdateBookRequest,
};
use crate::domain::{
    entities::book::BookEntity,
    repositories::book_repository::BookRepository,
    value_objects::{isbn13::Isbn13, money::Money},
};

/// BookService — handles application-level operations for books
pub struct BookService {
    book_repo: Arc<dyn BookRepository>,
}

impl BookService {
    pub fn new(book_repo: Arc<dyn BookRepository>) -> Self {
        Self { book_repo }
    }

    /// Create a new book
    pub async fn create_book(&self, req: CreateBookRequest) -> Result<BookResponse> {
        let isbn = Isbn13::new(&req.isbn)?;
        let decimal = Decimal::from_f64(req.price).unwrap_or(Decimal::ZERO);
        let price = Money::from_decimal(decimal)?;

        let book = BookEntity::new(
            isbn.clone(),
            req.title,
            Some(req.author),
            Some(req.synopsis),
            price,
        )?;

        self.book_repo.save(&book).await?;
        Ok(BookResponse::from(book))
    }

    /// Find book by ISBN
    pub async fn get_book_by_isbn(&self, isbn: String) -> Result<Option<BookResponse>> {
        let isbn = Isbn13::new(&isbn)?;
        let book_opt = self.book_repo.find_by_isbn(&isbn).await?;
        Ok(book_opt.map(BookResponse::from))
    }

    /// Get all books (with pagination)
    pub async fn get_all_books(&self, limit: u32, offset: u32) -> Result<Vec<BookResponse>> {
        let books = self.book_repo.find_all(limit, offset).await?;
        Ok(books.into_iter().map(BookResponse::from).collect())
    }

    /// Update a book
    pub async fn update_book(&self, isbn: String, req: UpdateBookRequest) -> Result<()> {
        let isbn_obj = Isbn13::new(&isbn)?;
        let mut book = match self.book_repo.find_by_isbn(&isbn_obj).await? {
            Some(b) => b,
            None => anyhow::bail!("Book not found"),
        };

        if let Some(title) = req.title {
            book.title = title;
        }
        if let Some(author) = req.author {
            book.author = Some(author);
        }
        if let Some(synopsis) = req.synopsis {
            book.synopsis = Some(synopsis);
        }
        if let Some(price) = req.price {
            let decimal = Decimal::from_f64(price).unwrap_or(Decimal::ZERO);
            book.price = Money::from_decimal(decimal)?;
        }
        if let Some(is_active) = req.is_active {
            book.is_active = is_active;
        }

        self.book_repo.save(&book).await
    }

    /// Delete a book
    pub async fn delete_book(&self, isbn: String) -> Result<()> {
        let isbn = Isbn13::new(&isbn)?;
        self.book_repo.delete(&isbn).await
    }
}
