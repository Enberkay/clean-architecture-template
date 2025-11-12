use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
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

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub age: i32,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub password: String,
    pub sex: String,
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
}
