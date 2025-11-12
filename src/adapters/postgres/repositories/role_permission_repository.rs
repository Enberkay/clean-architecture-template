use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::repositories::role_permission_repository::RolePermissionRepository;

/// PostgreSQL implementation of RolePermissionRepository.
pub struct PostgresRolePermissionRepository {
    pool: PgPool,
}

impl PostgresRolePermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RolePermissionRepository for PostgresRolePermissionRepository {
    async fn assign_permissions(&self, role_id: i32, permission_ids: &[i32]) -> Result<()> {
        for &perm_id in permission_ids {
            sqlx::query!(
                r#"
                INSERT INTO role_permissions (role_id, permission_id, assigned_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT (role_id, permission_id) DO NOTHING
                "#,
                role_id,
                perm_id
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn remove_permissions(&self, role_id: i32, permission_ids: &[i32]) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM role_permissions
            WHERE role_id = $1
              AND permission_id = ANY($2)
            "#,
            role_id,
            permission_ids
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn clear_permissions(&self, role_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM role_permissions
            WHERE role_id = $1
            "#,
            role_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_permissions_for_role(&self, role_id: i32) -> Result<Vec<i32>> {
        let results = sqlx::query!(
            r#"
            SELECT permission_id
            FROM role_permissions
            WHERE role_id = $1
            ORDER BY permission_id ASC
            "#,
            role_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(|r| r.permission_id).collect())
    }
}
