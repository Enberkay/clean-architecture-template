use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;

use crate::domain::{
    entities::role::RoleEntity,
    repositories::role_repository::RoleRepository,
};
use crate::infrastructure::postgres::models::role_model::RoleModel;

pub struct PostgresRoleRepository {
    pool: PgPool,
}

impl PostgresRoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    /// ดึง role ทั้งหมด
    async fn find_all(&self) -> Result<Vec<RoleEntity>> {
        let results = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(RoleEntity::from).collect())
    }

    /// ดึง role ตาม id
    async fn find_by_id(&self, id: i32) -> Result<Option<RoleEntity>> {
        let result = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(RoleEntity::from))
    }

    /// บันทึก role ใหม่
    async fn save(&self, role: &RoleEntity) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO roles (name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#,
            role.name,
            role.description,
            role.created_at,
            role.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// อัปเดตข้อมูล role
    async fn update(&self, role: &RoleEntity) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE roles
            SET name = $1,
                description = $2,
                updated_at = $3
            WHERE id = $4
            "#,
            role.name,
            role.description,
            role.updated_at,
            role.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// ลบ role ตาม id
    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
