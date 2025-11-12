use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::{
    entities::user::UserEntity,
    value_objects::email_address::EmailAddress,
};

// ======================
// UserModel (SQLx)
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: i32,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub password: String,

    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}



// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<UserModel> for UserEntity {
    fn from(model: UserModel) -> Self {
        Self {
            id: model.id,
            first_name: model.fname,
            last_name: model.lname,
            email: EmailAddress::new(&model.email)
                .expect("Invalid email format in database"),
            age: model.age,
            sex: model.sex,
            phone: model.phone,
            password: model.password,

            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<UserEntity> for UserModel {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            fname: entity.first_name,
            lname: entity.last_name,
            email: entity.email.as_str().to_string(),
            age: entity.age,
            sex: entity.sex,
            phone: entity.phone,
            password: entity.password,

            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
