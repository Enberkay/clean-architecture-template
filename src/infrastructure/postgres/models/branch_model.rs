use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::entities::branch::BranchEntity;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BranchModel {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<BranchModel> for BranchEntity {
    fn from(model: BranchModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            address: model.address,
            phone: model.phone,
            created_at: model.created_at,
        }
    }
}

impl From<BranchEntity> for BranchModel {
    fn from(entity: BranchEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            address: entity.address,
            phone: entity.phone,
            created_at: entity.created_at,
        }
    }
}
