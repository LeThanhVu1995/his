use sqlx::{Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    let pool = Pool::connect(url).await?;
    Ok(pool)
}
