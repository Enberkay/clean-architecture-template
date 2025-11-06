use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::domain::entities::branch::BranchEntity;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBranchRequest {
    #[validate(length(min = 2, max = 100, message = "Branch name must be 2-100 characters"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Address too long (max 500 chars)"))]
    pub address: Option<String>,
    
    #[validate(length(min = 6, max = 20, message = "Phone must be 6-20 characters"))]
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBranchRequest {
    #[validate(length(min = 2, max = 100, message = "Branch name must be 2-100 characters"))]
    pub name: Option<String>,
    
    #[validate(length(max = 500, message = "Address too long (max 500 chars)"))]
    pub address: Option<String>,
    
    #[validate(length(min = 6, max = 20, message = "Phone must be 6-20 characters"))]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BranchEntity> for BranchResponse {
    fn from(entity: BranchEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            address: entity.address,
            phone: entity.phone,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
