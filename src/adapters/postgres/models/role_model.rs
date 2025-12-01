use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::domain::{
    entities::role::RoleEntity,
    value_objects::{
        role_name::RoleName,
        role_description::RoleDescription,
    },
};

// ======================
// RoleModel (SQLx)
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoleModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<RoleModel> for RoleEntity {
    fn from(model: RoleModel) -> Self {
        Self {
            id: model.id,
            // แก้ไข: ใช้ RoleName::new และ .expect() เพราะข้อมูลใน DB ควรจะถูกต้องอยู่แล้ว
            name: RoleName::new(model.name).expect("Invalid role name in database"),
            
            // แก้ไข: จัดการ Option และ RoleDescription
            description: model.description.map(|d| {
                RoleDescription::new(d).expect("Invalid role description in database")
            }),
            
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<RoleEntity> for RoleModel {
    fn from(entity: RoleEntity) -> Self {
        Self {
            id: entity.id,
            // แก้ไข: ดึง string ออกมาจาก Value Object
            name: entity.name.as_str().to_string(),
            
            // แก้ไข: map เอา string ออกมาจาก Option<Value Object>
            description: entity.description.map(|d| d.as_str().to_string()),
            
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}


// alt
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use anyhow::Result; // จำเป็นสำหรับ TryFrom ถ้าจะเปิดใช้

// use crate::domain::{
//     entities::role::RoleEntity,
//     value_objects::{
//         role_name::RoleName,
//         role_description::RoleDescription,
//     },
// };

// // ======================
// // RoleModel (SQLx)
// // ======================

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct RoleModel {
//     pub id: i32,
//     pub name: String,
//     pub description: Option<String>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// // ==================================
// // Mapping: Domain Entity -> DB Model
// // ==================================

// impl From<RoleEntity> for RoleModel {
//     fn from(entity: RoleEntity) -> Self {
//         Self {
//             id: entity.id,
//             name: entity.name.as_str().to_string(),
//             description: entity.description.map(|d| d.as_str().to_string()),
//             created_at: entity.created_at,
//             updated_at: entity.updated_at,
//         }
//     }
// }

// // ==================================
// // Mapping: DB Model -> Domain Entity
// // ==================================

// // แบบที่ 1: ใช้ From (Panic ถ้าข้อมูลผิด) - สะดวก แต่ต้องมั่นใจข้อมูลใน DB
// impl From<RoleModel> for RoleEntity {
//     fn from(model: RoleModel) -> Self {
//         Self {
//             id: model.id,
//             // .expect จะทำให้โปรแกรม Crash ทันทีถ้าข้อมูลใน DB ไม่ผ่าน Validation
//             name: RoleName::new(model.name)
//                 .expect("Data corruption: Invalid role name in database"),
            
//             description: model.description.map(|d| {
//                 RoleDescription::new(d)
//                     .expect("Data corruption: Invalid role description in database")
//             }),
            
//             created_at: model.created_at,
//             updated_at: model.updated_at,
//         }
//     }
// }

// // ================================================================
// // 🔽 Reference: แบบปลอดภัย (TryFrom) 🔽
// // วิธีใช้: ถ้าจะใช้ ให้ Comment block "impl From" ด้านบนออก แล้วเปิด code นี้แทน
// // ================================================================

// /*
// impl TryFrom<RoleModel> for RoleEntity {
//     type Error = anyhow::Error;

//     fn try_from(model: RoleModel) -> Result<Self, Self::Error> {
//         // ใช้ ? เพื่อส่ง Error กลับไปแทนการ Panic
//         let name_vo = RoleName::new(model.name)?;
        
//         let desc_vo = match model.description {
//             Some(d) => Some(RoleDescription::new(d)?),
//             None => None,
//         };

//         Ok(Self {
//             id: model.id,
//             name: name_vo,
//             description: desc_vo,
//             created_at: model.created_at,
//             updated_at: model.updated_at,
//         })
//     }
// }

// // ⚠️ ข้อควรระวังถ้าเปลี่ยนมาใช้ TryFrom:
// // ใน Repository คุณต้องแก้โค้ดตอน map ด้วย เช่น:
// //
// // จากเดิม:
// // .map(RoleEntity::from).collect()
// //
// // ต้องเปลี่ยนเป็น:
// // .map(RoleEntity::try_from).collect::<Result<Vec<_>, _>>()?
// */