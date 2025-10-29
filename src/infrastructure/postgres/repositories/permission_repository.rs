use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::{
    entities::permission::PermissionEntity,
    repositories::permission_repository::PermissionRepository,
};
use crate::infrastructure::postgres::models::permission_model::PermissionModel;

pub struct PostgresPermissionRepository {
    pool: PgPool,
}

impl PostgresPermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PermissionRepository for PostgresPermissionRepository {
    async fn find_all(&self) -> Result<Vec<PermissionEntity>> {
        let results = sqlx::query_as::<_, PermissionModel>(
            r#"
            SELECT id, name, description, created_at
            FROM permissions
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(PermissionEntity::from).collect())
    }

    async fn save(&self, permission: &PermissionEntity) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO permissions (id, name, description, created_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id)
            DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description
            "#,
            permission.id,
            permission.name,
            permission.description,
            permission.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM permissions
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
