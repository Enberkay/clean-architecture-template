use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait JwtService: Send + Sync {
    async fn generate_access_token(&self, user_id: i32, roles: &[String]) -> Result<String>;
    async fn generate_refresh_token(&self, user_id: i32) -> Result<String>;
    async fn validate_access_token(&self, token: &str) -> Result<Claims>;
    async fn validate_refresh_token(&self, token: &str) -> Result<i32>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct JwtTokenService {
    access_encoding_key: EncodingKey,
    access_decoding_key: DecodingKey,
    refresh_encoding_key: EncodingKey,
    refresh_decoding_key: DecodingKey,
    access_token_expiry: Duration,
    refresh_token_expiry: Duration,
}

impl JwtTokenService {
    pub fn new(
        jwt_secret: &str,
        refresh_secret: &str,
        access_token_expiry_minutes: i64,
        refresh_token_expiry_days: i64,
    ) -> Self {
        Self {
            access_encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            access_decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            refresh_encoding_key: EncodingKey::from_secret(refresh_secret.as_bytes()),
            refresh_decoding_key: DecodingKey::from_secret(refresh_secret.as_bytes()),
            access_token_expiry: Duration::minutes(access_token_expiry_minutes),
            refresh_token_expiry: Duration::days(refresh_token_expiry_days),
        }
    }
}

#[async_trait]
impl JwtService for JwtTokenService {
    async fn generate_access_token(&self, user_id: i32, roles: &[String]) -> Result<String> {
        let now = Utc::now();
        let exp = (now + self.access_token_expiry).timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            roles: roles.to_vec(),
            exp,
            iat: now.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.access_encoding_key)
            .context("Failed to generate access token")
    }

    async fn generate_refresh_token(&self, user_id: i32) -> Result<String> {
        let now = Utc::now();
        let exp = (now + self.refresh_token_expiry).timestamp() as usize;

        let claims = RefreshClaims {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.refresh_encoding_key)
            .context("Failed to generate refresh token")
    }

    async fn validate_access_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.access_decoding_key, &validation)
            .context("Invalid access token")?;

        Ok(token_data.claims)
    }

    async fn validate_refresh_token(&self, token: &str) -> Result<i32> {
        let validation = Validation::default();
        let token_data = decode::<RefreshClaims>(token, &self.refresh_decoding_key, &validation)
            .context("Invalid refresh token")?;

        token_data.claims.sub.parse::<i32>().context("Invalid user ID in refresh token")
    }
}