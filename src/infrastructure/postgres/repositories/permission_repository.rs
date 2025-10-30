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
        let rows = sqlx::query_as::<_, PermissionModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM permissions
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(PermissionEntity::from).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PermissionEntity>> {
        let row = sqlx::query_as::<_, PermissionModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM permissions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(PermissionEntity::from))
    }

    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<PermissionEntity>> {
        let rows = sqlx::query_as::<_, PermissionModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM permissions
            WHERE id = ANY($1)
            ORDER BY id ASC
            "#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(PermissionEntity::from).collect())
    }

    async fn save(&self, permission: &PermissionEntity) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO permissions (name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#,
            permission.name,
            permission.description,
            permission.created_at,
            permission.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, permission: &PermissionEntity) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE permissions
            SET name = $1,
                description = $2,
                updated_at = NOW()
            WHERE id = $3
            "#,
            permission.name,
            permission.description,
            permission.id
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
