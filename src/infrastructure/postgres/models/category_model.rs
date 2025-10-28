use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::{
    infrastructure::postgres::schema::categories,
    domain::entities::category::CategoryEntity,
};

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = categories)]
#[diesel(primary_key(id))]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<CategoryModel> for CategoryEntity {
    fn from(model: CategoryModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<CategoryEntity> for CategoryModel {
    fn from(entity: CategoryEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
