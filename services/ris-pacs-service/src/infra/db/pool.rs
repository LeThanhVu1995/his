use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::config::Settings;

pub type PgPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}