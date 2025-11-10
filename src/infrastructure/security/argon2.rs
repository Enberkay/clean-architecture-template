use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use async_trait::async_trait;

/// Password hashing interface
#[async_trait]
pub trait PasswordService: Send + Sync {
    async fn hash_password(&self, password: &str) -> Result<String>;
    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool>;
}

#[derive(Debug, Clone)]
pub struct Argon2PasswordHasher {
    memory_cost: u32,
    time_cost: u32,
    parallelism: u32,
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self {
            memory_cost: 4096,
            time_cost: 3,
            parallelism: 1,
        }
    }
}

impl Argon2PasswordHasher {
    pub fn new(memory_cost: u32, time_cost: u32, parallelism: u32) -> Self {
        Self {
            memory_cost,
            time_cost,
            parallelism,
        }
    }
}

#[async_trait]
impl PasswordService for Argon2PasswordHasher {
    async fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let bytes_password = password.as_bytes();

        // Create Argon2 with custom parameters
        let params = Params::new(
            self.memory_cost,
            self.time_cost,
            self.parallelism,
            Some(32), // output length
        ).map_err(|e| anyhow::anyhow!("Invalid Argon2 parameters: {}", e))?;

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            params,
        );

        let result = argon2
            .hash_password(bytes_password, &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;

        Ok(result.to_string())
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        let bytes_password = password.as_bytes();

        // Create Argon2 with custom parameters for verification
        let params = Params::new(
            self.memory_cost,
            self.time_cost,
            self.parallelism,
            Some(32),
        ).map_err(|e| anyhow::anyhow!("Invalid Argon2 parameters: {}", e))?;

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            params,
        );

        Ok(argon2.verify_password(bytes_password, &parsed_hash).is_ok())
    }
}
