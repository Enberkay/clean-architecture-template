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
    async fn find_all(&self) -> Result<Vec<BranchEntity>> {
        let results = sqlx::query_as::<_, BranchModel>(
            r#"
            SELECT id, name, address, phone, created_at, updated_at
            FROM branches
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(BranchEntity::from).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<BranchEntity>> {
        let result = sqlx::query_as::<_, BranchModel>(
            r#"
            SELECT id, name, address, phone, created_at, updated_at
            FROM branches
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(BranchEntity::from))
    }

    async fn save(&self, branch: &BranchEntity) -> Result<i32> {
        let result = sqlx::query!(
            r#"
            INSERT INTO branches (name, address, phone, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
            &branch.name,
            branch.address.as_ref(),
            branch.phone.as_ref(),
            &branch.created_at,
            &branch.updated_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.id)
    }

    async fn update(
        &self,
        id: i32,
        name: Option<String>,
        address: Option<String>,
        phone: Option<String>,
    ) -> Result<BranchEntity> {
        let result = sqlx::query_as!(
            BranchModel,
            r#"
            UPDATE branches
            SET
                name = COALESCE($1, name),
                address = COALESCE($2, address),
                phone = COALESCE($3, phone),
                updated_at = NOW()
            WHERE id = $4
            RETURNING id, name, address, phone, created_at, updated_at
            "#,
            name.as_ref(),
            address.as_ref(),
            phone.as_ref(),
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(BranchEntity::from(result))
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM branches
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
