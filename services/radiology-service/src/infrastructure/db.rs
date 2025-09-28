use sqlx::{Pool, Postgres};

pub type _PgPool = Pool<Postgres>;

pub async fn _connect(url: &str) -> anyhow::Result<_PgPool> {
    Ok(sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await?)
}
