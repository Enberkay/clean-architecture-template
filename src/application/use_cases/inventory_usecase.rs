use std::sync::Arc;

use crate::application::dtos::inventory_dto::{
    InventoryResponse,
    CreateInventoryRequest,
    UpdateInventoryRequest,
};
use crate::domain::{
    entities::inventory::InventoryEntity,
    repositories::inventory_repository::InventoryRepository,
    value_objects::isbn13::Isbn13,
};

/// InventoryService — handles inventory logic at application level
pub struct InventoryUseCase {
    inventory_repo: Arc<dyn InventoryRepository>,
}

impl InventoryUseCase {
    pub fn new(inventory_repo: Arc<dyn InventoryRepository>) -> Self {
        Self { inventory_repo }
    }

    /// Create or update stock for a given branch + ISBN
    pub async fn upsert_inventory(&self, req: CreateInventoryRequest) -> Result<InventoryResponse> {
        let isbn = Isbn13::new(&req.book_isbn)?;
        let new_inv = InventoryEntity::new(req.branch_id, isbn.clone(), req.quantity)?;

        // save or update existing record
        self.inventory_repo.save(&new_inv).await?;
        Ok(InventoryResponse::from(new_inv))
    }

    /// Find inventory for a branch and ISBN
    pub async fn get_inventory(&self, branch_id: i32, isbn_str: &str) -> Result<Option<InventoryResponse>> {
        let isbn = Isbn13::new(isbn_str)?;
        let inv_opt = self.inventory_repo.find(branch_id, &isbn).await?;
        Ok(inv_opt.map(InventoryResponse::from))
    }

    /// Update quantity manually (e.g. stock correction)
    pub async fn update_quantity(&self, branch_id: i32, isbn_str: &str, req: UpdateInventoryRequest) -> Result<()> {
        let isbn = Isbn13::new(isbn_str)?;
        let mut inv = match self.inventory_repo.find(branch_id, &isbn).await? {
            Some(i) => i,
            None => anyhow::bail!("Inventory not found"),
        };

        if let Some(qty) = req.quantity {
            inv.set_quantity(qty)?;
        }

        self.inventory_repo.update(&inv).await
    }

    /// Increase / Decrease stock
    pub async fn adjust_stock(&self, branch_id: i32, isbn_str: &str, delta: i32) -> Result<()> {
        let isbn = Isbn13::new(isbn_str)?;
        let mut inv = match self.inventory_repo.find(branch_id, &isbn).await? {
            Some(i) => i,
            None => anyhow::bail!("Inventory not found"),
        };

        if delta > 0 {
            inv.increase(delta as u32)?;
        } else if delta < 0 {
            inv.decrease((-delta) as u32)?;
        }

        self.inventory_repo.update(&inv).await
    }
}
