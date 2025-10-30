use serde::{Serialize, Deserialize};
use crate::domain::entities::category::CategoryEntity;

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<CategoryEntity> for CategoryResponse {
    fn from(entity: CategoryEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
        }
    }
}
