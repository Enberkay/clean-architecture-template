use serde::{Deserialize, Serialize};
use validator::Validate;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 50, message = "First name must be 2-50 characters"))]
    pub fname: String,

    #[validate(length(min = 2, max = 50, message = "Last name must be 2-50 characters"))]
    pub lname: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    // Simple rule: just require minimum length
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[validate(range(min = 1, max = 120, message = "Age must be between 1 and 120"))]
    pub age: i32,

    #[validate(length(min = 1, max = 20, message = "Sex must be 1-20 characters"))]
    pub sex: String,

    #[validate(length(min = 6, max = 20, message = "Phone must be 6-20 characters"))]
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
}
