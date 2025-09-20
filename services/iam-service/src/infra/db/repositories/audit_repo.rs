use crate::infra::db::pool::PgPool;

pub async fn insert_login(
    db: &PgPool,
    user_id: &str,
    username: Option<&str>,
    ip_addr: Option<&str>,
    user_agent: Option<&str>,
    success: bool,
) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO audit_user_login (user_id, username, ip_addr, user_agent, success)
           VALUES ($1,$2,$3,$4,$5)"#
    )
    .bind(user_id)
    .bind(username)
    .bind(ip_addr)
    .bind(user_agent)
    .bind(success)
    .execute(db)
    .await?;
    Ok(())
}
