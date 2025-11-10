use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::receipt::ReceiptEntity,
    repositories::receipt_repository::ReceiptRepository,
    value_objects::receipt_code::ReceiptCode,
};
use crate::infrastructure::postgres::models::receipt_model::ReceiptModel;

pub struct PostgresReceiptRepository {
    pool: PgPool,
}

impl PostgresReceiptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReceiptRepository for PostgresReceiptRepository {
    async fnOption<ReceiptEntity>> {
        let result = sqlx::query_as::<_, ReceiptModel>(
            r#"
            SELECT id, receipt_code, type_name, reference_id, source,
                   user_id, branch_id, total_amount, payment_method,
                   payment_ref, issued_at, status, created_at, updated_at
            FROM receipts
            WHERE receipt_code = $1
            "#,
        )
        .bind(code.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(ReceiptEntity::from))
    }

    async fnVec<ReceiptEntity>> {
        let results = sqlx::query_as::<_, ReceiptModel>(
            r#"
            SELECT id, receipt_code, type_name, reference_id, source,
                   user_id, branch_id, total_amount, payment_method,
                   payment_ref, issued_at, status, created_at, updated_at
            FROM receipts
            WHERE reference_id = $1
            ORDER BY issued_at DESC
            "#,
        )
        .bind(reference_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(ReceiptEntity::from).collect())
    }

    async fn()> {
        sqlx::query!(
            r#"
            INSERT INTO receipts
                (id, receipt_code, type_name, reference_id, source,
                 user_id, branch_id, total_amount, payment_method,
                 payment_ref, issued_at, status, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (id)
            DO UPDATE SET
                receipt_code = EXCLUDED.receipt_code,
                type_name = EXCLUDED.type_name,
                reference_id = EXCLUDED.reference_id,
                source = EXCLUDED.source,
                user_id = EXCLUDED.user_id,
                branch_id = EXCLUDED.branch_id,
                total_amount = EXCLUDED.total_amount,
                payment_method = EXCLUDED.payment_method,
                payment_ref = EXCLUDED.payment_ref,
                issued_at = EXCLUDED.issued_at,
                status = EXCLUDED.status,
                updated_at = EXCLUDED.updated_at
            "#,
            receipt.id,
            receipt.receipt_code.to_string(),
            receipt.type_name,
            receipt.reference_id,
            receipt.source,
            receipt.user_id,
            receipt.branch_id,
            receipt.total_amount.value(),
            receipt.payment_method,
            receipt.payment_ref,
            receipt.issued_at,
            receipt.status,
            receipt.created_at,
            receipt.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
