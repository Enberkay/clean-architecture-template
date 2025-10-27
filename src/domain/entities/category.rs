use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CategoryEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CategoryEntity {
    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, desc: Option<String>) {
        self.description = desc;
        self.updated_at = Utc::now();
    }
}
