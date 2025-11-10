use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::book_image::BookImageEntity,
    repositories::book_image_repository::BookImageRepository,
    value_objects::isbn13::Isbn13,
};
use crate::infrastructure::postgres::models::book_image_model::BookImageModel;

pub struct PostgresBookImageRepository {
    pool: PgPool,
}

impl PostgresBookImageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookImageRepository for PostgresBookImageRepository {
    async fn find_by_book(&self, isbn: &Isbn13) > {
        let models = sqlx::query_as::<_, BookImageModel>(
            r#"
            SELECT id, book_isbn, image_url, image_type, sort_order, created_at
            FROM book_images
            WHERE book_isbn = $1
            ORDER BY sort_order ASC
            "#,
        )
        .bind(isbn.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(models.into_iter().map(BookImageEntity::from).collect())
    }

    async fn add(&self, image: &BookImageEntity) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO book_images (book_isbn, image_url, image_type, sort_order, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            image.book_isbn.to_string(),
            image.image_url,
            image.image_type,
            image.sort_order,
            image.created_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM book_images WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
