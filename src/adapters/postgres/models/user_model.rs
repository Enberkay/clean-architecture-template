use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::{
    entities::user::UserEntity,
    value_objects::{
        age::Age,
        email_address::EmailAddress,
        password::Password,
        person_name::PersonName,
        phone_number::PhoneNumber,
    },
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
            first_name: PersonName::new(model.fname).expect("Invalid first name in database"),
            last_name: PersonName::new(model.lname).expect("Invalid last name in database"),
            email: EmailAddress::new(&model.email).expect("Invalid email in database"),
            age: Age::new(model.age).expect("Invalid age in database"),
            sex: model.sex,
            phone: PhoneNumber::new(model.phone).expect("Invalid phone in database"),
            password: Password::new(model.password).expect("Invalid password in database"),
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
            fname: entity.first_name.as_str().to_string(),
            lname: entity.last_name.as_str().to_string(),
            email: entity.email.as_str().to_string(),
            age: entity.age.value(),
            sex: entity.sex,
            phone: entity.phone.as_str().to_string(),
            password: entity.password.as_str().to_string(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

//alt
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// // use anyhow::Result; // จำเป็นถ้าจะเปิดใช้ TryFrom

// use crate::domain::{
//     entities::user::UserEntity,
//     value_objects::{
//         age::Age,
//         email_address::EmailAddress,
//         password::Password,
//         person_name::PersonName,
//         phone_number::PhoneNumber,
//     },
// };

// // ======================
// // UserModel (SQLx)
// // ======================

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct UserModel {
//     pub id: i32,
//     pub fname: String,
//     pub lname: String,
//     pub email: String,
//     pub age: i32,
//     pub sex: String,
//     pub phone: String,
//     pub password: String,
//     pub is_active: bool,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// // ==================================
// // Mapping: Domain Entity -> DB Model
// // ==================================

// impl From<UserEntity> for UserModel {
//     fn from(entity: UserEntity) -> Self {
//         Self {
//             id: entity.id,
//             fname: entity.first_name.as_str().to_string(),
//             lname: entity.last_name.as_str().to_string(),
//             email: entity.email.as_str().to_string(),
//             age: entity.age.value(),
//             sex: entity.sex,
//             phone: entity.phone.as_str().to_string(),
//             password: entity.password.as_str().to_string(),
//             is_active: entity.is_active,
//             created_at: entity.created_at,
//             updated_at: entity.updated_at,
//         }
//     }
// }

// // ==================================
// // Mapping: DB Model -> Domain Entity
// // ==================================

// // แบบที่ 1: ใช้ From (Panic ถ้าข้อมูลผิด) - สำหรับกรณีมั่นใจใน Data Integrity
// impl From<UserModel> for UserEntity {
//     fn from(model: UserModel) -> Self {
//         Self {
//             id: model.id,
//             first_name: PersonName::new(model.fname)
//                 .expect("Data corruption: Invalid first name in database"),
//             last_name: PersonName::new(model.lname)
//                 .expect("Data corruption: Invalid last name in database"),
//             email: EmailAddress::new(&model.email)
//                 .expect("Data corruption: Invalid email in database"),
//             age: Age::new(model.age)
//                 .expect("Data corruption: Invalid age in database"),
//             sex: model.sex,
//             phone: PhoneNumber::new(model.phone)
//                 .expect("Data corruption: Invalid phone in database"),
//             password: Password::new(model.password)
//                 .expect("Data corruption: Invalid password in database"),
//             is_active: model.is_active,
//             created_at: model.created_at,
//             updated_at: model.updated_at,
//         }
//     }
// }

// // ================================================================
// // Reference: แบบปลอดภัย (TryFrom)
// // วิธีใช้: ถ้าจะใช้ ให้ Comment block "impl From" ด้านบนออก แล้วเปิด code นี้แทน
// // และอย่าลืม uncomment "use anyhow::Result;" ด้านบนสุด
// // ================================================================

// /*
// impl TryFrom<UserModel> for UserEntity {
//     type Error = anyhow::Error;

//     fn try_from(model: UserModel) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: model.id,
//             // ใช้ ? เพื่อส่ง Error กลับไปแทนการ Panic
//             first_name: PersonName::new(model.fname)?,
//             last_name: PersonName::new(model.lname)?,
//             email: EmailAddress::new(&model.email)?,
//             age: Age::new(model.age)?,
//             sex: model.sex,
//             phone: PhoneNumber::new(model.phone)?,
//             password: Password::new(model.password)?,
//             is_active: model.is_active,
//             created_at: model.created_at,
//             updated_at: model.updated_at,
//         })
//     }
// }

// // ⚠️ หมายเหตุการใช้งานใน Repository:
// //
// // จากเดิม:
// // .map(UserEntity::from).collect()
// //
// // ต้องเปลี่ยนเป็น:
// // .map(UserEntity::try_from).collect::<Result<Vec<_>, _>>()?
// */