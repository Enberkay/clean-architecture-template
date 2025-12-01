use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::{
    entities::{user::UserEntity, role::RoleEntity},
    repositories::user_repository::UserRepository,
};
use crate::adapters::postgres::models::{user_model::UserModel, role_model::RoleModel};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, fname, lname, email, age, sex, phone, password,
                   is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        // UserModel -> UserEntity (แปลงผ่าน From/TryFrom ที่เราทำไว้)
        Ok(result.map(UserEntity::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, fname, lname, email, age, sex, phone, password,
                   is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(UserEntity::from))
    }

    async fn find_all(&self) -> Result<Vec<UserEntity>> {
        let results = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, fname, lname, email, age, sex, phone, password,
                   is_active, created_at, updated_at
            FROM users
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(UserEntity::from).collect())
    }

    async fn save(&self, user: &UserEntity) -> Result<i32> {
        let row = sqlx::query(
            r#"
            INSERT INTO users
                (fname, lname, email, age, sex, phone, password,
                 is_active, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#,
        )
        // ดึงค่าจาก Value Objects
        .bind(user.first_name.as_str())
        .bind(user.last_name.as_str())
        .bind(user.email.as_str())
        .bind(user.age.value())
        .bind(&user.sex)
        .bind(user.phone.as_str())
        .bind(user.password.as_str())
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get("id")?)
    }

    //แก้ไข: รับ Entity ทั้งก้อน เพื่อ update state ล่าสุดลง DB
    async fn update(&self, user: &UserEntity) -> Result<UserEntity> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"
            UPDATE users
            SET
                fname = $1,
                lname = $2,
                email = $3,
                age = $4,
                sex = $5,
                phone = $6,
                is_active = $7,
                updated_at = $8
            WHERE id = $9
            RETURNING id, fname, lname, email, age, sex, phone, password,
                      is_active, created_at, updated_at
            "#,
        )
        .bind(user.first_name.as_str())
        .bind(user.last_name.as_str())
        .bind(user.email.as_str())
        .bind(user.age.value())
        .bind(&user.sex)
        .bind(user.phone.as_str())
        .bind(user.is_active)
        .bind(user.updated_at) // เวลาอัปเดตถูกเปลี่ยนมาจาก Domain logic แล้ว
        .bind(user.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(UserEntity::from(result))
    }

    //เพิ่ม: Method สำหรับเปลี่ยน Password โดยเฉพาะ
    async fn update_password(&self, id: i32, new_password_hash: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET password = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(new_password_hash)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn assign_roles(&self, user_id: i32, role_ids: &[i32]) -> Result<()> {
        for &role_id in role_ids {
            sqlx::query(
                r#"
                INSERT INTO user_roles (user_id, role_id, assigned_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT (user_id, role_id) DO NOTHING
                "#,
            )
            .bind(user_id)
            .bind(role_id)
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn remove_roles(&self, user_id: i32, role_ids: &[i32]) -> Result<()> {
        sqlx::query("DELETE FROM user_roles WHERE user_id = $1 AND role_id = ANY($2)")
            .bind(user_id)
            .bind(role_ids)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_roles(&self, user_id: i32) -> Result<Vec<RoleEntity>> {
        let results = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT r.id, r.name, r.description, r.created_at, r.updated_at
            FROM roles r
            INNER JOIN user_roles ur ON ur.role_id = r.id
            WHERE ur.user_id = $1
            ORDER BY r.id ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(RoleEntity::from).collect())
    }
}