use std::sync::Arc;
use validator::Validate;

use anyhow::{Result, anyhow};
use crate::application::dtos::branch_dto::{
    BranchResponse, CreateBranchRequest, UpdateBranchRequest,
};
use crate::domain::{
    entities::branch::BranchEntity,
    repositories::branch_repository::BranchRepository,
};

/// BranchService — application-level orchestration for Branch entity
pub struct BranchUseCase {
    branch_repo: Arc<dyn BranchRepository>,
}

impl BranchUseCase {
    pub fn new(branch_repo: Arc<dyn BranchRepository>) -> Self {
        Self { branch_repo }
    }

    /// Create a new branch
    pub async fn create_branch(
        &self,
        req: CreateBranchRequest,
    ) -> Result<BranchResponse> {
        req.validate()
            .map_err(|e| anyhow!(e.to_string()))?;

        let mut branch = BranchEntity::new(req.name, req.address, req.phone)
            .map_err(|e| anyhow!(e.to_string()))?;

        let branch_id = self
            .branch_repo
            .save(&branch)
            .await
            .map_err(|e| anyhow!(format!("Failed to save branch: {}", e)))?;

        branch.id = branch_id;
        Ok(BranchResponse::from(branch))
    }

    /// Get a branch by ID
    pub async fn get_branch_by_id(
        &self,
        id: i32,
    ) -> Result<Option<BranchResponse>> {
        let branch_opt = self
            .branch_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch branch: {}", e)))?;
        Ok(branch_opt.map(BranchResponse::from))
    }

    /// Get all branches
    pub async fn get_all_branches(&self) -> Result<Vec<BranchResponse>> {
        let branches = self
            .branch_repo
            .find_all()
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch branches: {}", e)))?;
        Ok(branches.into_iter().map(BranchResponse::from).collect())
    }

    /// Update branch info
    pub async fn update_branch(
        &self,
        id: i32,
        req: UpdateBranchRequest,
    ) -> Result<BranchResponse> {
        req.validate()
            .map_err(|e| anyhow!(e.to_string()))?;

        // ตรวจสอบว่ามี branch นี้จริงไหม
        let exists = self
            .branch_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch branch: {}", e)))?;
        if exists.is_none() {
            return Err(anyhow!("Branch not found"));
        }

        // เรียกใช้ repository.update() โดยตรง (partial update)
        let updated_branch = self
            .branch_repo
            .update(id, req.name, req.address, req.phone)
            .await
            .map_err(|e| {
                if e.to_string().contains("duplicate key value") {
                    anyhow!("Branch name already exists")
                } else {
                    anyhow!(format!("Failed to update branch: {}", e))
                }
            })?;

        Ok(BranchResponse::from(updated_branch))
    }

    /// Delete branch
    pub async fn delete_branch(&self, id: i32) -> Result<BranchResponse> {
        let branch = match self
            .branch_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch branch: {}", e)))?
        {
            Some(b) => b,
            None => return Err(anyhow!("Branch not found")),
        };

        self.branch_repo
            .delete(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to delete branch: {}", e)))?;

        Ok(BranchResponse::from(branch))
    }
}
