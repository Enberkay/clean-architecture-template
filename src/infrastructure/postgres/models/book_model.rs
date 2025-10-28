use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use bigdecimal::BigDecimal;

use crate::{
    infrastructure::postgres::schema::books,
    domain::{
        entities::book::BookEntity,
        value_objects::{isbn13::Isbn13, money::Money},
    },
};

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = books)]
#[diesel(primary_key(isbn))]
pub struct BookModel {
    pub isbn: String,
    pub title: String,
    pub author: Option<String>,
    pub synopsis: Option<String>,
    pub price: BigDecimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// =============================
// Mapping between Model ↔ Entity
// =============================

impl From<BookModel> for BookEntity {
    fn from(model: BookModel) -> Self {
        Self {
            isbn: Isbn13::new(&model.isbn).expect("Invalid ISBN-13"),
            title: model.title,
            author: model.author,
            synopsis: model.synopsis,
            price: Money::from_bigdecimal(&model.price).expect("Invalid price"),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<BookEntity> for BookModel {
    fn from(entity: BookEntity) -> Self {
        Self {
            isbn: entity.isbn.to_string(),
            title: entity.title,
            author: entity.author,
            synopsis: entity.synopsis,
            price: entity.price.to_bigdecimal(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
