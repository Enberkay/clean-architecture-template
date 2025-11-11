use anyhow::{Result, anyhow};
use crate::domain::value_objects::email_address::EmailAddress;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: EmailAddress,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub password: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserEntity {
    /// Creates a new validated User entity.
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        age: i32,
        sex: String,
        phone: String,
        password: String,
    ) -> Result<Self> {
        Self::validate_name(&first_name)?;
        Self::validate_name(&last_name)?;
        Self::validate_age(age)?;
        Self::validate_phone(&phone)?;
        Self::validate_password(&password)?;

        let email = EmailAddress::new(&email)?;
        let now = Utc::now();

        Ok(Self {
            id: 0,
            first_name: first_name.trim().to_string(),
            last_name: last_name.trim().to_string(),
            email,
            age,
            sex: sex.trim().to_uppercase(),
            phone: phone.trim().to_string(),
            password,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    /// Returns the user's full name.
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Deactivates the user.
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Activates the user.
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Updates the user's email (revalidates)
    pub fn update_email(&mut self, new_email: String) -> Result<()> {
        let new = EmailAddress::new(&new_email)?;
        self.email = new;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Changes the user's password (hashed)
    pub fn change_password(&mut self, hashed_password: String) -> Result<()> {
        Self::validate_password(&hashed_password)?;
        self.password = hashed_password;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update user phone
    pub fn update_phone(&mut self, new_phone: String) -> Result<()> {
        Self::validate_phone(&new_phone)?;
        self.phone = new_phone;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Validate the entire entity
    pub fn validate(&self) -> Result<()> {
        Self::validate_name(&self.first_name)?;
        Self::validate_name(&self.last_name)?;
        Self::validate_age(self.age)?;
        Self::validate_phone(&self.phone)?;
        Ok(())
    }

    // ----------------------------
    // Internal validation helpers
    // ----------------------------

    fn validate_name(name: &str) -> Result<()> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Name cannot be empty"));
        }
        if trimmed.len() > 100 {
            return Err(anyhow!("Name too long (max 100 chars)"));
        }
        Ok(())
    }

    fn validate_age(age: i32) -> Result<()> {
        if age < 1 || age > 120 {
            return Err(anyhow!("Age must be between 1 and 120"));
        }
        Ok(())
    }

    fn validate_phone(phone: &str) -> Result<()> {
        let trimmed = phone.trim();
        if trimmed.len() < 6 || trimmed.len() > 20 {
            return Err(anyhow!("Phone number length invalid"));
        }
        if !trimmed.chars().all(|c| c.is_ascii_digit() || c == '+' || c == '-') {
            return Err(anyhow!("Phone number must contain only digits, '+', or '-'"));
        }
        Ok(())
    }

    fn validate_password(password: &str) -> Result<()> {
        if password.trim().len() < 8 {
            return Err(anyhow!("Password must be at least 8 characters long"));
        }
        Ok(())
    }
}
