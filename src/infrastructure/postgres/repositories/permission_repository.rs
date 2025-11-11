use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

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
            SELECT id, name, description, created_at, updated_at
            FROM permissions
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(PermissionEntity::from).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PermissionEntity>> {
        let result = sqlx::query_as::<_, PermissionModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM permissions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(PermissionEntity::from))
    }

    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<PermissionEntity>> {
        let results = sqlx::query_as::<_, PermissionModel>(
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

        Ok(results.into_iter().map(PermissionEntity::from).collect())
    }

    async fn save(&self, permission: &PermissionEntity) -> Result<i32> {
        let result = sqlx::query!(
            r#"
            INSERT INTO permissions (name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            &permission.name,
            permission.description.as_ref(),
            &permission.created_at,
            &permission.updated_at,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.id)
    }

    async fn update(&self, id: i32, name: Option<String>, description: Option<String>) -> Result<PermissionEntity> {
        let result = sqlx::query_as!(
            PermissionModel,
            r#"
            UPDATE permissions
            SET 
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                updated_at = NOW()
            WHERE id = $3
            RETURNING id, name, description, created_at, updated_at
            "#,
            name.as_ref(),
            description.as_ref(),
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(PermissionEntity::from(result))
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
