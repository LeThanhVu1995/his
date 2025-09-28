use app_web::prelude::*;

pub async fn connect(database_url: &str) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    sqlx::PgPool::connect(database_url).await.map_err(|e| e.into())
}
