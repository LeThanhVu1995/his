pub async fn connect(database_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(16)
        .connect(database_url)
        .await
}
