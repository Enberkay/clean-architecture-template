use crate::config::config_model::AppConfig;
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
}
