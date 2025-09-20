use sqlx::{Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    Ok(sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await?)
}
