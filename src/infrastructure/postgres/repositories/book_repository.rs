use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::book::BookEntity,
    repositories::book_repository::BookRepository,
    value_objects::isbn13::Isbn13,
};
use crate::infrastructure::postgres::models::book_model::BookModel;

pub struct PostgresBookRepository {
    pool: PgPool,
}

impl PostgresBookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookRepository for PostgresBookRepository {
    async fn find_by_isbn(&self, isbn: &Isbn13) > {
        let result = sqlx::query_as::<_, BookModel>(
            r#"
            SELECT isbn, title, author, synopsis, price, is_active, created_at, updated_at
            FROM books
            WHERE isbn = $1
            "#,
        )
        .bind(isbn.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(BookEntity::from))
    }

    async fn find_all(&self, limit: u32, offset: u32) > {
        let results = sqlx::query_as::<_, BookModel>(
            r#"
            SELECT isbn, title, author, synopsis, price, is_active, created_at, updated_at
            FROM books
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(BookEntity::from).collect())
    }

    async fn save(&self, book: &BookEntity) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO books (isbn, title, author, synopsis, price, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (isbn)
            DO UPDATE SET
                title = EXCLUDED.title,
                author = EXCLUDED.author,
                synopsis = EXCLUDED.synopsis,
                price = EXCLUDED.price,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
            book.isbn.to_string(),
            book.title,
            book.author,
            book.synopsis,
            book.price.value(),
            book.is_active,
            book.created_at,
            book.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, isbn: &Isbn13) -> anyhow::Result<()> {
        sqlx::query!(
            r#"DELETE FROM books WHERE isbn = $1"#,
            isbn.to_string()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
