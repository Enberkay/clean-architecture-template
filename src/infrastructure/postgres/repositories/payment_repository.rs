use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::payment::PaymentEntity,
    repositories::payment_repository::PaymentRepository,
};
use crate::infrastructure::postgres::models::payment_model::PaymentModel;

pub struct PostgresPaymentRepository {
    pool: PgPool,
}

impl PostgresPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentRepository for PostgresPaymentRepository {
    async fnOption<PaymentEntity>> {
        let result = sqlx::query_as::<_, PaymentModel>(
            r#"
            SELECT id, order_id, sale_id, payment_method, transaction_ref,
                   amount, status, created_at, updated_at
            FROM payments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(PaymentEntity::from))
    }

    async fnVec<PaymentEntity>> {
        let results = sqlx::query_as::<_, PaymentModel>(
            r#"
            SELECT id, order_id, sale_id, payment_method, transaction_ref,
                   amount, status, created_at, updated_at
            FROM payments
            WHERE order_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(order_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(PaymentEntity::from).collect())
    }

    async fn()> {
        sqlx::query!(
            r#"
            INSERT INTO payments
                (id, order_id, sale_id, payment_method, transaction_ref,
                 amount, status, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id)
            DO UPDATE SET
                order_id = EXCLUDED.order_id,
                sale_id = EXCLUDED.sale_id,
                payment_method = EXCLUDED.payment_method,
                transaction_ref = EXCLUDED.transaction_ref,
                amount = EXCLUDED.amount,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
            payment.id,
            payment.order_id,
            payment.sale_id,
            payment.payment_method,
            payment.transaction_ref,
            payment.amount.value(),
            payment.status,
            payment.created_at,
            payment.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
