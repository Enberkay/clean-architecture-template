use crate::config::config_model::*;
use anyhow::{Context, Result, anyhow};

pub fn load() -> Result<AppConfig> {
    fn required_env(key: &str) -> Result<String> {
        std::env::var(key)
            .with_context(|| format!("MISSING required environment variable: {}", key))
    }

    fn parse_env<T: std::str::FromStr>(key: &str) -> Result<T>
    where
        T::Err: std::fmt::Display + Send + Sync + 'static,
    {
        let value = required_env(key)?;
        value
            .parse::<T>()
            .map_err(|e| anyhow!("Invalid value for {}: {}", key, e))
    }

    fn parse_bool_env(key: &str) -> Result<bool> {
        match required_env(key)?.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(true),
            "false" | "0" | "no" => Ok(false),
            v => Err(anyhow!("Invalid boolean for {}: {}", key, v)),
        }
    }

    // Server
    let server = Server {
        port: parse_env("SERVER_PORT")?,
        body_limit: parse_env("SERVER_BODY_LIMIT")?,
        timeout_seconds: parse_env("SERVER_TIMEOUT")?,
        cors_allowed_origins: required_env("SERVER_CORS_ALLOWED_ORIGINS")?
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
    };

    // Database
    let database = Database {
        url: required_env("DATABASE_URL")?,
    };

    // Redis
    let redis = Redis {
        url: required_env("REDIS_URL")?,
        max_connections: parse_env("REDIS_MAX_CONNECTIONS")?,
        refresh_token_expiry_days: parse_env("REDIS_REFRESH_TOKEN_EXPIRY_DAYS")?,
    };

    // JWT
    let jwt = JwtConfig {
        access_token_expiry_minutes: parse_env("JWT_ACCESS_TOKEN_EXPIRY_MINUTES")?,
    };

    // Environment
    let environment: Environment = required_env("ENVIRONMENT")?.parse()?;

    // Compose full config
    let config = AppConfig {
        server,
        database,
        redis,
        jwt,
        environment,
    };

    config.validate()?;
    Ok(config)
}
