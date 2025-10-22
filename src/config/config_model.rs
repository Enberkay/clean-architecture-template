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

// UsersSecret
#[derive(Debug, Clone)]
pub struct UsersSecret {
    pub secret: String,
    pub refresh_secret: String,
}

impl UsersSecret {
    pub fn validate(&self) -> Result<()> {
        if self.secret.len() < 32 {
            anyhow::bail!("JWT_USERS_SECRET must be ≥ 32 chars.")
        }
        if self.refresh_secret.len() < 32 {
            anyhow::bail!("JWT_USERS_REFRESH_SECRET must be ≥ 32 chars.")
        }
        Ok(())
    }
}

// JwtConfig
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_expiry_minutes: u64,
}

impl JwtConfig {
    pub fn validate(&self) -> Result<()> {
        if self.access_token_expiry_minutes == 0 {
            anyhow::bail!("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must be > 0.")
        }
        Ok(())
    }
}

//Environment
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl std::str::FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "staging" => Ok(Self::Staging),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(anyhow::anyhow!("Invalid ENVIRONMENT: {}", s)),
        }
    }
}

// SecurityConfig
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub rate_limit_requests_per_minute: u32,
    pub argon2_memory_cost: u32,
    pub argon2_time_cost: u32,
    pub argon2_parallelism: u32,
}

impl SecurityConfig {
    pub fn validate(&self) -> Result<()> {
        if self.argon2_memory_cost < 1024 {
            anyhow::bail!("ARGON2_MEMORY_COST too low (<1024 KB)")
        }
        if self.argon2_time_cost == 0 {
            anyhow::bail!("ARGON2_TIME_COST must be > 0")
        }
        if self.argon2_parallelism == 0 {
            anyhow::bail!("ARGON2_PARALLELISM must be > 0")
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProductionConfig {
    pub https_redirect: bool,
    pub trust_proxy: bool,
}
