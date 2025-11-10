use std::sync::Arc;

use crate::application::dtos::category_dto::{
    CategoryResponse,
    CreateCategoryRequest,
    UpdateCategoryRequest,
};

use crate::domain::{
    entities::category::CategoryEntity,
    repositories::category_repository::CategoryRepository,
};

/// CategoryService — Handles use cases for Category entity
pub struct CategoryUseCase {
    category_repo: Arc<dyn CategoryRepository>,
}

impl CategoryUseCase {
    pub fn new(category_repo: Arc<dyn CategoryRepository>) -> Self {
        Self { category_repo }
    }

    /// Create new category
    pub async fn create_category(&self, req: CreateCategoryRequest) -> Result<CategoryResponse> {
        let category = CategoryEntity::new(req.name, req.description)?;

        self.category_repo.save(&category).await?;
        Ok(CategoryResponse::from(category))
    }

    /// Get category by id
    pub async fn get_category_by_id(&self, id: i32) -> Result<Option<CategoryResponse>> {
        let category_opt = self.category_repo.find_by_id(id).await?;
        Ok(category_opt.map(CategoryResponse::from))
    }

    /// Get all categories
    pub async fn get_all_categories(&self) -> Result<Vec<CategoryResponse>> {
        let categories = self.category_repo.find_all().await?;
        Ok(categories.into_iter().map(CategoryResponse::from).collect())
    }

    /// Update category
    pub async fn update_category(&self, id: i32, req: UpdateCategoryRequest) -> Result<()> {
        let mut category = match self.category_repo.find_by_id(id).await? {
            Some(c) => c,
            None => anyhow::bail!("Category not found"),
        };

        if let Some(name) = req.name {
            category.name = name;
        }
        if let Some(description) = req.description {
            category.description = Some(description);
        }

        self.category_repo.save(&category).await
    }

    /// Delete category
    pub async fn delete_category(&self, id: i32) -> Result<()> {
        self.category_repo.delete(id).await
    }
}
