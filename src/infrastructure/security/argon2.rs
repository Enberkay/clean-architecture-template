use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use async_trait::async_trait;

// Argon2 configuration variables
const MEMORY_COST: u32 = 65536;    // 64 MB
const TIME_COST: u32 = 3;          // 3 iterations
const PARALLELISM: u32 = 4;        // 4 threads

#[async_trait]
pub trait PasswordService: Send + Sync {
    async fn hash_password(&self, password: &str) -> anyhow::Result<String>;
    async fn verify_password(&self, password: &str, hash: &str) -> anyhow::Result<bool>;
}

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PasswordService for Argon2PasswordHasher {
    async fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(MEMORY_COST, TIME_COST, PARALLELISM, Some(32))?;
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        
        let result = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;
        
        Ok(result.to_string())
    }

    async fn verify_password(&self, password: &str, hash: &str) -> anyhow::Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        
        let params = Params::new(MEMORY_COST, TIME_COST, PARALLELISM, Some(32))?;
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
        
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}