use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::role::RoleEntity,
    entities::permission::PermissionEntity,
    repositories::role_repository::RoleRepository,
};
use crate::infrastructure::postgres::models::{
    role_model::{RoleModel, RolePermissionModel},
    permission_model::PermissionModel,
};

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
    async fnVec<RoleEntity>> {
        // Get all roles first
        let roles = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            ORDER BY id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        // Get all role permissions
        let role_permissions = sqlx::query_as::<_, RolePermissionModel>(
            r#"
            SELECT rp.role_id, p.id, p.name, p.description, p.created_at, p.updated_at
            FROM role_permissions rp
            JOIN permissions p ON rp.permission_id = p.id
            ORDER BY rp.role_id, p.id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        // Build entities with permissions
        let mut roles_with_permissions = Vec::new();
        for role in roles {
            let permissions: Vec<PermissionEntity> = role_permissions
                .iter()
                .filter(|rp| rp.role_id == role.id)
                .map(|rp| PermissionEntity {
                    id: rp.id,
                    name: rp.name.clone(),
                    description: rp.description.clone(),
                    created_at: rp.created_at,
                    updated_at: rp.updated_at,
                })
                .collect();

            let mut role_entity = RoleEntity::from(role);
            role_entity.set_permissions(permissions).map_err(|e| anyhow::anyhow!("Invalid permissions: {}", e))?;
            roles_with_permissions.push(role_entity);
        }

        Ok(roles_with_permissions)
    }

    async fnOption<RoleEntity>> {
        // Get role
        let role = sqlx::query_as::<_, RoleModel>(
            r#"
            SELECT id, name, description, created_at, updated_at
            FROM roles
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(role_model) = role {
            // Get permissions for this role
            let permissions = sqlx::query_as::<_, PermissionModel>(
                r#"
                SELECT p.id, p.name, p.description, p.created_at, p.updated_at
                FROM role_permissions rp
                JOIN permissions p ON rp.permission_id = p.id
                WHERE rp.role_id = $1
                ORDER BY p.id
                "#,
            )
            .bind(id)
            .fetch_all(&self.pool)
            .await?;

            let mut role_entity = RoleEntity::from(role_model);
            role_entity.set_permissions(permissions.into_iter().map(PermissionEntity::from).collect())
                .map_err(|e| anyhow::anyhow!("Invalid permissions: {}", e))?;
            
            Ok(Some(role_entity))
        } else {
            Ok(None)
        }
    }

    async fnVec<RoleEntity>> {
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

    async fni32> {
        let result = sqlx::query!(
            r#"
            INSERT INTO roles (name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            role.name,
            role.description,
            role.created_at,
            role.created_at // ใช้ created_at สำหรับ updated_at ถ้ายังไม่มี
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.id)
    }

    async fnRoleEntity> {
        let result = sqlx::query_as!(
            RoleModel,
            r#"
            UPDATE roles
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

        Ok(RoleEntity::from(result))
    }

    async fn()> {
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
