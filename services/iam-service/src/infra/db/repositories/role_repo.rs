// iam-service src/infra/db/repositories/role_repo.rs placeholder
use uuid::Uuid;
use sqlx::Row;
use crate::infra::db::pool::PgPool;
use crate::domain::entities::role::Role;

pub async fn list(db: &PgPool) -> Result<Vec<Role>, app_error::AppError> {
    let rows = sqlx::query(
        r#"SELECT id, code, name, created_at FROM iam_roles ORDER BY name"#
    )
    .fetch_all(db)
    .await?;

    let items: Vec<Role> = rows.into_iter().map(|row| Role {
        id: row.get("id"),
        code: row.get("code"),
        name: row.get("name"),
        created_at: row.get("created_at"),
    }).collect();

    Ok(items)
}

pub async fn assign(db: &PgPool, user_id: Uuid, role_id: Uuid) -> Result<(), app_error::AppError> {
    sqlx::query(
        r#"INSERT INTO iam_user_roles (user_id, role_id) VALUES ($1,$2)
           ON CONFLICT DO NOTHING"#
    )
    .bind(user_id)
    .bind(role_id)
    .execute(db)
    .await?;
    Ok(())
}
