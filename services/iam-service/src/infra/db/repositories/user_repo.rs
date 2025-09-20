use sqlx::Row;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::entities::user::User;
use crate::infra::db::pool::PgPool;
use app_core::prelude::*;

pub async fn create(db: &PgPool, username: &str, full_name: Option<&str>, email: Option<&str>) -> Result<User, app_error::AppError> {
    let id = Uuid::new_v4();
    let row = sqlx::query(
        r#"INSERT INTO iam_users (id, username, full_name, email)
           VALUES ($1,$2,$3,$4)
           RETURNING id, username, full_name, email, locked, created_at, updated_at"#
    )
    .bind(id)
    .bind(username)
    .bind(full_name)
    .bind(email)
    .fetch_one(db)
    .await?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        full_name: row.get("full_name"),
        email: row.get("email"),
        locked: row.get("locked"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<User, app_error::AppError> {
    let row = sqlx::query(
        r#"SELECT id, username, full_name, email, locked, created_at, updated_at
           FROM iam_users WHERE id = $1"#
    )
    .bind(id)
    .fetch_one(db)
    .await
    .or_not_found("user", Some(id.to_string()))?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        full_name: row.get("full_name"),
        email: row.get("email"),
        locked: row.get("locked"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn list(db: &PgPool, offset: i64, limit: i64) -> Result<(Vec<User>, i64), app_error::AppError> {
    let rows = sqlx::query(
        r#"SELECT id, username, full_name, email, locked, created_at, updated_at
           FROM iam_users ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(db)
    .await?;

    let items: Vec<User> = rows.into_iter().map(|row| User {
        id: row.get("id"),
        username: row.get("username"),
        full_name: row.get("full_name"),
        email: row.get("email"),
        locked: row.get("locked"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }).collect();

    let total = sqlx::query(r#"SELECT COUNT(*) as count FROM iam_users"#)
        .fetch_one(db)
        .await?
        .get::<i64, _>("count");

    Ok((items, total))
}

pub async fn update(db: &PgPool, id: Uuid, full_name: Option<String>, email: Option<String>, locked: Option<bool>) -> Result<User, app_error::AppError> {
    let row = sqlx::query(
        r#"UPDATE iam_users SET
              full_name = COALESCE($2, full_name),
              email     = COALESCE($3, email),
              locked    = COALESCE($4, locked),
              updated_at = now()
           WHERE id = $1
           RETURNING id, username, full_name, email, locked, created_at, updated_at"#
    )
    .bind(id)
    .bind(full_name)
    .bind(email)
    .bind(locked)
    .fetch_one(db)
    .await
    .or_not_found("user", Some(id.to_string()))?;

    Ok(User {
        id: row.get("id"),
        username: row.get("username"),
        full_name: row.get("full_name"),
        email: row.get("email"),
        locked: row.get("locked"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}
