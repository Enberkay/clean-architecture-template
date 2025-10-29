use anyhow::{Context, Result};
use sqlx::PgPool;

pub type PgPoolSquad = PgPool;

pub async fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let pool = PgPool::connect(database_url)
        .await
        .context("Failed to create database pool")?;
    Ok(pool)
}
