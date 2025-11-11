
pub struct AppConfig {
    pub server: Server,
    pub database: Database,
    pub users_secret: UsersSecret,
    pub jwt: JwtConfig,
    pub environment: Environment,
}

impl AppConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        self.server.validate()?;
        self.database.validate()?;
        self.jwt.validate()?;
        self.users_secret.validate()?;
        self.validate_cross_configuration()?;
        Ok(())
    }

    fn validate_cross_configuration(&self) -> anyhow::Result<()> {
        if self.jwt.access_token_expiry_minutes > 60 * 24 {
            anyhow::bail!("JWT access token expiry should not exceed 24 hours")
        }

        match self.environment {
            Environment::Production => {
                if self.server.cors_allowed_origins.contains(&"*".to_string()) {
                    anyhow::bail!("CORS cannot allow all origins (*) in production")
                }
            }
            Environment::Staging => {
                if self.server.cors_allowed_origins.contains(&"*".to_string()) {
                    anyhow::bail!("CORS cannot allow all origins (*) in staging")
                }
            }
            Environment::Development => {}
        }
        Ok(())
    }
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
    pub fn validate(&self) -> anyhow::Result<()> {
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
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.url.is_empty() {
            anyhow::bail!("DATABASE_URL connot be empty.")
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
    pub fn validate(&self) -> anyhow::Result<()> {
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
    pub refresh_token_expiry_days: u64,
}

impl JwtConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.access_token_expiry_minutes == 0 {
            anyhow::bail!("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must be > 0.")
        }
        if self.refresh_token_expiry_days == 0 {
            anyhow::bail!("JWT_REFRESH_TOKEN_EXPIRY_DAYS must be > 0.")
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

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "staging" => Ok(Self::Staging),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(anyhow::anyhow!("Invalid ENVIRONMENT: {}", s)),
        }
    }
}
