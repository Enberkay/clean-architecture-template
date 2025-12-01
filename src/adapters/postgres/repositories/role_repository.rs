use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::{
    entities::role::RoleEntity,
    repositories::role_repository::RoleRepository,
};
use crate::adapters::postgres::models::role_model::RoleModel;

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

    //เพิ่ม Implementation นี้ตาม Trait ใหม่
    async fn find_by_name(&self, name: &str) -> Result<Option<RoleEntity>> {
        let result = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE name = $1
            "#,
        )
        .bind(name) // name ใน DB เป็น String ปกติเทียบกับ &str ได้เลย
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(RoleEntity::from))
    }

    async fn find_by_ids(&self, ids: &[i32]) -> Result<Vec<RoleEntity>> {
        let results = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE id = ANY($1)
            ORDER BY id ASC
            "#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(RoleEntity::from).collect())
    }

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

    async fn save(&self, role: &RoleEntity) -> Result<i32> {
        let row = sqlx::query(
            r#"
            INSERT INTO roles
                (name, description, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        //แก้ไข: ดึงค่า string จาก Value Object
        .bind(role.name.as_str()) 
        //แก้ไข: map Option<ValueObject> -> Option<String>
        .bind(role.description.as_ref().map(|d| d.as_str())) 
        .bind(role.created_at)
        .bind(role.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get("id")?)
    }

    //แก้ไข Signature และ SQL: รับ Entity มาทั้งก้อนแล้วบันทึกสถานะล่าสุดลงไป
    async fn update(&self, role: &RoleEntity) -> Result<RoleEntity> {
        let result = sqlx::query_as::<_, RoleModel>(
            r#"
            UPDATE roles
            SET
                name = $1,
                description = $2,
                updated_at = $3
            WHERE id = $4
            RETURNING id, name, description, created_at, updated_at
            "#,
        )
        .bind(role.name.as_str()) // Update name
        .bind(role.description.as_ref().map(|d| d.as_str())) // Update description
        .bind(role.updated_at) // Update timestamp (ที่เปลี่ยนมาจาก Domain Logic)
        .bind(role.id) // Where ID
        .fetch_one(&self.pool)
        .await?;

        Ok(RoleEntity::from(result))
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}