use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::inventory::InventoryEntity,
    repositories::inventory_repository::InventoryRepository,
    value_objects::isbn13::Isbn13,
};
use crate::infrastructure::postgres::models::inventory_model::InventoryModel;

pub struct PostgresInventoryRepository {
    pool: PgPool,
}

impl PostgresInventoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryRepository for PostgresInventoryRepository {
    async fnOption<InventoryEntity>> {
        let result = sqlx::query_as::<_, InventoryModel>(
            r#"
            SELECT branch_id, book_isbn, quantity, updated_at
            FROM inventories
            WHERE branch_id = $1 AND book_isbn = $2
            "#,
        )
        .bind(branch_id)
        .bind(isbn.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(InventoryEntity::from))
    }

    async fn()> {
        sqlx::query!(
            r#"
            INSERT INTO inventories (branch_id, book_isbn, quantity, updated_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (branch_id, book_isbn)
            DO UPDATE SET
                quantity = EXCLUDED.quantity,
                updated_at = EXCLUDED.updated_at
            "#,
            inventory.branch_id,
            inventory.book_isbn.to_string(),
            inventory.quantity.value() as i32,
            inventory.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn()> {
        sqlx::query!(
            r#"
            UPDATE inventories
            SET quantity = $1, updated_at = $2
            WHERE branch_id = $3 AND book_isbn = $4
            "#,
            inventory.quantity.value() as i32,
            inventory.updated_at,
            inventory.branch_id,
            inventory.book_isbn.to_string()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
