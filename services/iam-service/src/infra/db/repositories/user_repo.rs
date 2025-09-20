use sqlx::Row;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::entities::user::User;
use crate::infra::db::pool::PgPool;
use app_core::prelude::*;

pub async fn create(db: &PgPool, username: &str, full_name: Option<&str>, email: Option<&str>) -> Result<User, app_error::AppError> {
    let id = Uuid::new_v4();
    let rec = sqlx::query_as!(
        User,
        r#"INSERT INTO iam_users (id, username, full_name, email)
           VALUES ($1,$2,$3,$4)
           RETURNING id, username, full_name, email, locked, created_at, updated_at"#,
        id, username, full_name, email
    )
    .fetch_one(db)
    .await?;
    Ok(rec)
}

pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<User, app_error::AppError> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, full_name, email, locked, created_at, updated_at
           FROM iam_users WHERE id = $1"#,
        id
    )
    .fetch_one(db)
    .await
    .or_not_found("user", Some(id.to_string()))
}

pub async fn list(db: &PgPool, offset: i64, limit: i64) -> Result<(Vec<User>, i64), app_error::AppError> {
    let items = sqlx::query_as!(
        User,
        r#"SELECT id, username, full_name, email, locked, created_at, updated_at
           FROM iam_users ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
        offset, limit
    )
    .fetch_all(db)
    .await?;

    let total = sqlx::query_scalar!(r#"SELECT COUNT(*) as "count!" FROM iam_users"#)
        .fetch_one(db)
        .await?;

    Ok((items, total.unwrap_or(0)))
}

pub async fn update(db: &PgPool, id: Uuid, full_name: Option<String>, email: Option<String>, locked: Option<bool>) -> Result<User, app_error::AppError> {
    let rec = sqlx::query_as!(
        User,
        r#"UPDATE iam_users SET
              full_name = COALESCE($2, full_name),
              email     = COALESCE($3, email),
              locked    = COALESCE($4, locked),
              updated_at = now()
           WHERE id = $1
           RETURNING id, username, full_name, email, locked, created_at, updated_at"#,
        id,
        full_name,
        email,
        locked
    )
    .fetch_one(db)
    .await
    .or_not_found("user", Some(id.to_string()))?;
    Ok(rec)
}
