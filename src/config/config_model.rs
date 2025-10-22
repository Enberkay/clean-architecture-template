use anyhow::Result;

pub struct AppConfig {
    pub server: Server,
    pub database: Database,
    pub redis: Redis,
    pub users_secret: UsersSecret,
    pub jwt: JwtConfig,
    pub environment: Environment,
    pub security: SecurityConfig,
    pub production: ProductionConfig,
}

// Server
#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout_seconds: u32,
    pub cors_allowed_origins: Vec<String>,
}

impl Server {
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            anyhow::bail!("Server port must be greater than 0.")
        }
        if self.body_limit == 0 {
            anyhow::bail!("Server body limit must be greater than 0.")
        }
        if self.timeout_seconds == 0 {
            anyhow::bail!("Server timeout must be greater than 0.")
        }
        for origin in &self.cors_allowed_origins {
            if origin == "*" && !origin.starts_with("http://") && !origin.starts_with("https://") {
                anyhow::bail!("Invalid CORS origin format: {}", origin)
            }
        }
        Ok(())
    }
}

// Database
#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

impl Database {
    pub fn validate(&self) -> Result<()> {
        if self.url.is_empty() {
            anyhow::bail!("DATABASE_URL connot be empty.")
        }
        Ok(())
    }
}

// Redis
#[derive(Debug, Clone)]
pub struct Redis {
    pub url: String,
    pub max_connections: u32,
    pub refresh_token_expiry_days: u64,
}

impl Redis {
    pub fn validate(&self) -> Result<()> {
        if self.url.is_empty() {
            anyhow::bail!("REDIS_URL cannot be empty.")
        }
        if self.max_connections == 0 {
            anyhow::bail!("REDIS_MAX_CONNECTIONS must be > 0.")
        }
        if self.refresh_token_expiry_days == 0 {
            anyhow::bail!("REDIS_REFRESH_TOKEN_EXPIRY_DAYS must be > 0.")
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UsersSecret {
    pub secret: String,
    pub refresh_secret: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_expiry_minutes: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub rate_limit_requests_per_minute: u32,
    pub argon2_memory_cost: u32,
    pub argon2_time_cost: u32,
    pub argon2_parallelism: u32,
}

#[derive(Debug, Clone)]
pub struct ProductionConfig {
    pub https_redirect: bool,
    pub trust_proxy: bool,
}
