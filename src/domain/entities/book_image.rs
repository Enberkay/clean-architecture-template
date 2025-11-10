use anyhow::{Result, anyhow};
use crate::domain::{
    value_objects::isbn13::Isbn13,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BookImageEntity {
    pub id: i32,
    pub book_isbn: Isbn13,
    pub image_url: String,
    pub image_type: String, // "COVER" | "PREVIEW" | "GALLERY"
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

impl BookImageEntity {
    /// Create a new book image with validation
    pub fn new(
        book_isbn: Isbn13,
        image_url: String,
        image_type: String,
        sort_order: i32,
    ) -> Result<Self> {
        Self::validate_type(&image_type)?;
        Self::validate_url(&image_url)?;

        Ok(Self {
            id: 0,
            book_isbn,
            image_url,
            image_type: image_type.to_uppercase(),
            sort_order,
            created_at: Utc::now(),
        })
    }

    /// Update image URL (e.g. when re-uploaded)
    pub fn update_url(&mut self, new_url: String) -> Result<()> {
        Self::validate_url(&new_url)?;
        self.image_url = new_url;
        Ok(())
    }

    /// Change image type (e.g. from PREVIEW → GALLERY)
    pub fn change_type(&mut self, new_type: &str) -> Result<()> {
        Self::validate_type(new_type)?;
        self.image_type = new_type.to_uppercase();
        Ok(())
    }

    /// Reorder image (useful for UI sorting)
    pub fn set_sort_order(&mut self, new_order: i32) {
        self.sort_order = new_order;
    }

    /// Quick check if this image is a cover image
    pub fn is_cover(&self) -> bool {
        self.image_type.eq_ignore_ascii_case("COVER")
    }

    // ==========================
    // Internal validators
    // ==========================

    fn validate_url(url: &str) -> Result<()> {
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return Err(anyhow!("Image URL must start with http or https"));
        }
        Ok(())
    }

    fn validate_type(image_type: &str) -> Result<()> {
        let valid_types = ["COVER", "PREVIEW", "GALLERY"];
        if !valid_types.contains(&image_type.to_uppercase().as_str()) {
            return Err(anyhow!(format!(
                "Invalid image type: {}",
                image_type
            )));
        }
        Ok(())
    }
}
