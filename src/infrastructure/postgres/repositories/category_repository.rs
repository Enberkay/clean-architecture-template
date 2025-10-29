use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::{
    entities::category::CategoryEntity,
    repositories::category_repository::CategoryRepository,
};
use crate::infrastructure::postgres::models::category_model::CategoryModel;

pub struct PostgresCategoryRepository {
    pool: PgPool,
}

impl PostgresCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<CategoryEntity>> {
        let result = sqlx::query_as::<_, CategoryModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM categories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(CategoryEntity::from))
    }

    async fn find_all(&self) -> Result<Vec<CategoryEntity>> {
        let results = sqlx::query_as::<_, CategoryModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM categories
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(CategoryEntity::from).collect())
    }

    async fn save(&self, category: &CategoryEntity) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO categories (id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id)
            DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                updated_at = EXCLUDED.updated_at
            "#,
            category.id,
            category.name,
            category.description,
            category.created_at,
            category.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM categories WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
