use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait JwtService: Send + Sync {
    async fn generate_access_token(&self, user_id: i32, roles: &[String], permissions: &[String]) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<i32>;
    async fn validate_access_token_claims(&self, token: &str) -> Result<Claims>;
    async fn generate_refresh_token(&self, user_id: i32) -> Result<String>;
    async fn validate_refresh_token(&self, token: &str) -> Result<i32>;

    fn get_refresh_secret(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

pub struct JwtTokenService {
    jwt_secret: String,
    refresh_secret: String,
    access_token_expiry_minutes: u64,
    refresh_token_expiry_days: u64,
}

impl JwtTokenService {
    pub fn new(jwt_secret: &str, refresh_secret: &str, access_token_expiry_minutes: u64, refresh_token_expiry_days: u64) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
            refresh_secret: refresh_secret.to_string(),
            access_token_expiry_minutes,
            refresh_token_expiry_days,
        }
    }
}

#[async_trait]
impl JwtService for JwtTokenService {
    async fn generate_access_token(&self, user_id: i32, roles: &[String], permissions: &[String]) -> Result<String> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            roles: roles.to_vec(),
            permissions: permissions.to_vec(),
            exp: now + ((self.access_token_expiry_minutes * 60) as usize),
            iat: now,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.jwt_secret.as_ref()))
            .map_err(|e| anyhow::anyhow!("Failed to create access token: {}", e))
    }

    async fn validate_token(&self, token: &str) -> Result<i32> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| anyhow::anyhow!("Failed to validate token: {}", e))?;

        token_data
            .claims
            .sub
            .parse::<i32>()
            .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))
    }

    async fn validate_access_token_claims(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| anyhow::anyhow!("Failed to validate access token: {}", e))?;

        Ok(token_data.claims)
    }

    async fn generate_refresh_token(&self, user_id: i32) -> Result<String> {
        let now = Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id.to_string(),
            roles: Vec::new(),
            permissions: Vec::new(),
            exp: now + ((self.refresh_token_expiry_days * 24 * 60 * 60) as usize), // configurable days
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_ref()),
        )
        .map_err(|e| anyhow::anyhow!("Failed to create refresh token: {}", e))
    }



    async fn validate_refresh_token(&self, token: &str) -> Result<i32> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_ref()),
            &validation,
        )
        .map_err(|e| anyhow::anyhow!("Failed to validate refresh token: {}", e))?;

        token_data
            .claims
            .sub
            .parse::<i32>()
            .map_err(|e| anyhow::anyhow!("Invalid user ID in refresh token: {}", e))
    }

    fn get_refresh_secret(&self) -> &str {
        &self.refresh_secret
    }
}

// Standalone function for middleware usage
pub fn validate_access_token_claims(token: &str, jwt_secret: &str) -> Result<Claims> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &validation,
    )
    .map_err(|e| anyhow::anyhow!("Failed to validate access token: {}", e))?;

    Ok(token_data.claims)
}
