use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::{
    age::Age,
    email_address::EmailAddress,
    password::Password,
    person_name::PersonName,
    phone_number::PhoneNumber,
};

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub first_name: PersonName,
    pub last_name: PersonName,
    pub email: EmailAddress,
    pub age: Age,
    pub sex: String,
    pub phone: PhoneNumber,
    pub password: Password,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserEntity {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        age: i32,
        sex: String,
        phone: String,
        password: String,
    ) -> Result<Self> {
        let now = Utc::now();

        Ok(Self {
            id: 0,
            first_name: PersonName::new(first_name)?,
            last_name: PersonName::new(last_name)?,
            email: EmailAddress::new(&email)?,
            age: Age::new(age)?,
            sex: sex.trim().to_uppercase(),
            phone: PhoneNumber::new(phone)?,
            password: Password::new(password)?,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn update_email(&mut self, new_email: String) -> Result<()> {
        self.email = EmailAddress::new(&new_email)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn change_password(&mut self, new_password: String) -> Result<()> {
        self.password = Password::new(new_password)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_phone(&mut self, new_phone: String) -> Result<()> {
        self.phone = PhoneNumber::new(new_phone)?;
        self.updated_at = Utc::now();
        Ok(())
    }
}