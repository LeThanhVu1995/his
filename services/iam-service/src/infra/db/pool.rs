// iam-service src/infra/db/pool.rs placeholder
use sqlx::{Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn make_pg_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = sqlx::PgPool::connect(database_url).await?;
    Ok(pool)
}
