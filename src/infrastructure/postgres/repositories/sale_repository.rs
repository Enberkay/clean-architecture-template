use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::sale::{SaleEntity, SaleItemEntity},
    repositories::sale_repository::SaleRepository,
};
use crate::infrastructure::postgres::models::{sale_model::SaleModel, sale_model::SaleItemModel};

pub struct PostgresSaleRepository {
    pool: PgPool,
}

impl PostgresSaleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SaleRepository for PostgresSaleRepository {
    async fnOption<SaleEntity>> {
        let result = sqlx::query_as::<_, SaleModel>(
            r#"
            SELECT id, employee_id, branch_id, sale_date,
                   total_amount, payment_method, created_at
            FROM sales
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(SaleEntity::from))
    }

    async fnVec<SaleItemEntity>> {
        let results = sqlx::query_as::<_, SaleItemModel>(
            r#"
            SELECT id, sale_id, book_isbn, book_title, book_author,
                   quantity, price_at_sale, subtotal, created_at
            FROM sale_items
            WHERE sale_id = $1
            ORDER BY id ASC
            "#,
        )
        .bind(sale_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(SaleItemEntity::from).collect())
    }

    async fn()> {
        sqlx::query!(
            r#"
            INSERT INTO sales (id, employee_id, branch_id, sale_date, total_amount, payment_method, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id)
            DO UPDATE SET
                employee_id = EXCLUDED.employee_id,
                branch_id = EXCLUDED.branch_id,
                sale_date = EXCLUDED.sale_date,
                total_amount = EXCLUDED.total_amount,
                payment_method = EXCLUDED.payment_method
            "#,
            sale.id,
            sale.employee_id,
            sale.branch_id,
            sale.sale_date,
            sale.total_amount.value(),
            sale.payment_method,
            sale.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn()> {
        for item in items {
            sqlx::query!(
                r#"
                INSERT INTO sale_items
                    (id, sale_id, book_isbn, book_title, book_author, quantity, price_at_sale, subtotal, created_at)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (id)
                DO UPDATE SET
                    book_title = EXCLUDED.book_title,
                    book_author = EXCLUDED.book_author,
                    quantity = EXCLUDED.quantity,
                    price_at_sale = EXCLUDED.price_at_sale,
                    subtotal = EXCLUDED.subtotal
                "#,
                item.id,
                item.sale_id,
                item.book_isbn,
                item.book_title,
                item.book_author,
                item.quantity,
                item.price_at_sale.value(),
                item.subtotal.value(),
                item.created_at
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!("DELETE FROM sale_items WHERE sale_id = $1", id)
            .execute(&mut *tx)
            .await?;

        sqlx::query!("DELETE FROM sales WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
