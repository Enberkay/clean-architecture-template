use async_trait::async_trait;
#[async_trait]
pub trait RolePermissionRepository: Send + Sync {
    /// Assign multiple permissions to a role.
    async fn assign_permissions(&self, role_id: i32, permission_ids: &[i32]) -> anyhow::Result<()>;

    /// Remove specific permissions from a role.
    async fn remove_permissions(&self, role_id: i32, permission_ids: &[i32]) -> anyhow::Result<()>;

    /// Remove all permissions for a given role.
    async fn clear_permissions(&self, role_id: i32) -> anyhow::Result<()>;

    /// Get all permission IDs assigned to a role.
    async fn get_permissions_for_role(&self, role_id: i32) -> anyhow::Result<Vec<i32>>;
}
