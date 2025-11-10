use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{
    entities::order::{OrderEntity, OrderItemEntity},
    repositories::order_repository::OrderRepository,
};
use crate::infrastructure::postgres::models::{
    order_model::{OrderModel, OrderItemModel},
};

pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Helper: Save items in transaction context
    async fn insert_items_tx(
        tx: &mut Transaction<'_, Postgres>,
        items: &[OrderItemEntity],
    ) -> Result<()> {
        for item in items {
            sqlx::query!(
                r#"
                INSERT INTO order_items
                    (order_id, book_isbn, book_title, book_author, quantity, price_at_purchase, subtotal, created_at)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
                item.order_id,
                item.book_isbn,
                item.book_title,
                item.book_author,
                item.quantity,
                item.price_at_purchase.value(),
                item.subtotal.value(),
                item.created_at
            )
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }
}

#[async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fnOption<OrderEntity>> {
        let result = sqlx::query_as::<_, OrderModel>(
            r#"
            SELECT id, user_id, order_date, status, source, total_amount,
                   shipping_address, created_at, updated_at
            FROM orders
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(OrderEntity::from))
    }

    async fnVec<OrderItemEntity>> {
        let results = sqlx::query_as::<_, OrderItemModel>(
            r#"
            SELECT id, order_id, book_isbn, book_title, book_author,
                   quantity, price_at_purchase, subtotal, created_at
            FROM order_items
            WHERE order_id = $1
            ORDER BY id ASC
            "#,
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(OrderItemEntity::from).collect())
    }

    async fn()> {
        sqlx::query!(
            r#"
            INSERT INTO orders
                (id, user_id, order_date, status, source, total_amount,
                 shipping_address, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id)
            DO UPDATE SET
                user_id = EXCLUDED.user_id,
                status = EXCLUDED.status,
                source = EXCLUDED.source,
                total_amount = EXCLUDED.total_amount,
                shipping_address = EXCLUDED.shipping_address,
                updated_at = EXCLUDED.updated_at
            "#,
            order.id,
            order.user_id,
            order.order_date,
            order.status,
            order.source,
            order.total_amount.value(),
            order.shipping_address,
            order.created_at,
            order.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn()> {
        let mut tx = self.pool.begin().await?;
        Self::insert_items_tx(&mut tx, items).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn()> {
        let mut tx = self.pool.begin().await?;

        // Delete items first (FK constraint)
        sqlx::query!("DELETE FROM order_items WHERE order_id = $1", id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!("DELETE FROM orders WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn()> {
        let mut tx = self.pool.begin().await?;

        // Save order
        sqlx::query!(
            r#"
            INSERT INTO orders
                (id, user_id, order_date, status, source, total_amount,
                 shipping_address, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id)
            DO UPDATE SET
                user_id = EXCLUDED.user_id,
                status = EXCLUDED.status,
                source = EXCLUDED.source,
                total_amount = EXCLUDED.total_amount,
                shipping_address = EXCLUDED.shipping_address,
                updated_at = EXCLUDED.updated_at
            "#,
            order.id,
            order.user_id,
            order.order_date,
            order.status,
            order.source,
            order.total_amount.value(),
            order.shipping_address,
            order.created_at,
            order.updated_at
        )
        .execute(&mut *tx)
        .await?;

        // Save items in same transaction
        Self::insert_items_tx(&mut tx, items).await?;

        tx.commit().await?;
        Ok(())
    }
}
