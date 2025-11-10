use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use hex;
use hmac::{Hmac, Mac};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use serde::{Deserialize, Serialize};
use sha2::Sha256;

/// JWT service interface
#[async_trait]
pub trait JwtService: Send + Sync {
    async fn generate_access_token(&self, user_id: i32, roles: &[String], permissions: &[String]) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<i32>;
    async fn generate_refresh_token(&self, user_id: i32, expiry_days: u64) -> Result<String>;
    async fn hash_token(&self, token: &str) -> Result<String>;
    fn get_refresh_secret(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,              // Subject (user_id)
    pub roles: Vec<String>,       // Roles ของ user
    pub permissions: Vec<String>, // Permissions ที่ผู้ใช้มี (จาก roles)
    pub exp: usize,               // Expiration timestamp
    pub iat: usize,               // Issued at timestamp
}

pub fn create_access_token(
    user_id: i32,
    roles: Vec<String>,
    permissions: Vec<String>,
    jwt_secret: &str,
    expiry_minutes: u64,
) -> Result<String> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        roles,
        permissions,
        exp: now + (expiry_minutes * 60) as usize, // Convert minutes to seconds
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .context("Failed to create access token")
}

pub fn create_refresh_token(
    user_id: i32,
    refresh_secret: &str,
    expiry_days: u64,
) -> Result<String> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        roles: Vec::new(), // Refresh tokens don't need roles/permissions
        permissions: Vec::new(),
        exp: now + (expiry_days * 24 * 60 * 60) as usize, // Convert days to seconds
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(refresh_secret.as_ref()),
    )
    .context("Failed to create refresh token")
}

pub fn validate_refresh_token(token: &str, refresh_secret: &str) -> Result<i32> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(refresh_secret.as_ref()),
        &validation,
    )
    .context("Failed to validate refresh token")?;

    let user_id = token_data
        .claims
        .sub
        .parse::<i32>()
        .context("Invalid user ID in refresh token")?;
    Ok(user_id)
}

#[derive(Debug, Clone)]
pub struct JwtTokenService {
    pub jwt_secret: String,
    pub hmac_secret: String,
    pub access_token_expiry_minutes: u64,
}

impl JwtTokenService {
    pub fn new(jwt_secret: &str, refresh_secret: &str, access_token_expiry_minutes: u64) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
            hmac_secret: refresh_secret.to_string(), // Use separate secret for refresh tokens
            access_token_expiry_minutes,
        }
    }
}

pub fn validate_access_token(token: &str, jwt_secret: &str) -> Result<i32> {
    let claims = validate_access_token_claims(token, jwt_secret)?;
    let user_id = claims
        .sub
        .parse::<i32>()
        .context("Invalid user ID in token")?;
    Ok(user_id)
}

pub fn validate_access_token_claims(token: &str, jwt_secret: &str) -> Result<Claims> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
    .context("Failed to validate access token")?;

    Ok(token_data.claims)
}

#[async_trait]
impl JwtService for JwtTokenService {
    async fn generate_access_token(
        &self,
        user_id: i32,
        roles: &[String],
        permissions: &[String],
    ) -> Result<String> {
        create_access_token(
            user_id,
            roles.to_vec(),
            permissions.to_vec(),
            &self.jwt_secret,
            self.access_token_expiry_minutes,
        )
    }

    async fn validate_token(&self, token: &str) -> Result<i32> {
        validate_access_token(token, &self.jwt_secret)
    }

    async fn generate_refresh_token(&self, user_id: i32, expiry_days: u64) -> Result<String> {
        create_refresh_token(user_id, &self.hmac_secret, expiry_days)
    }

    async fn hash_token(&self, token: &str) -> Result<String> {
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(self.hmac_secret.as_bytes())
            .context("Failed to create HMAC")?;

        mac.update(token.as_bytes());
        let result = mac.finalize();
        let hash_bytes = result.into_bytes();

        Ok(hex::encode(hash_bytes))
    }

    fn get_refresh_secret(&self) -> &str {
        &self.hmac_secret
    }
}
