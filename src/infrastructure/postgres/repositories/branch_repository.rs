use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::{
    entities::branch::BranchEntity,
    repositories::branch_repository::BranchRepository,
};
use crate::infrastructure::postgres::models::branch_model::BranchModel;

pub struct PostgresBranchRepository {
    pool: PgPool,
}

impl PostgresBranchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BranchRepository for PostgresBranchRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<BranchEntity>> {
        let result = sqlx::query_as::<_, BranchModel>(
            r#"
            SELECT id, name, address, phone, created_at
            FROM branches
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(BranchEntity::from))
    }

    async fn find_all(&self) -> Result<Vec<BranchEntity>> {
        let results = sqlx::query_as::<_, BranchModel>(
            r#"
            SELECT id, name, address, phone, created_at
            FROM branches
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(BranchEntity::from).collect())
    }

    async fn save(&self, branch: &BranchEntity) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO branches (id, name, address, phone, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id)
            DO UPDATE SET
                name = EXCLUDED.name,
                address = EXCLUDED.address,
                phone = EXCLUDED.phone
            "#,
            branch.id,
            branch.name,
            branch.address,
            branch.phone,
            branch.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM branches WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
