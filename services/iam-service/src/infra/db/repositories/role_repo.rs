// iam-service src/infra/db/repositories/role_repo.rs placeholder
use uuid::Uuid;
use crate::infra::db::pool::PgPool;
use crate::domain::entities::role::Role;

pub async fn list(db: &PgPool) -> Result<Vec<Role>, app_error::AppError> {
    let items = sqlx::query_as!(
        Role,
        r#"SELECT id, code, name, created_at FROM iam_roles ORDER BY name"#
    )
    .fetch_all(db)
    .await?;
    Ok(items)
}

pub async fn assign(db: &PgPool, user_id: Uuid, role_id: Uuid) -> Result<(), app_error::AppError> {
    sqlx::query!(
        r#"INSERT INTO iam_user_roles (user_id, role_id) VALUES ($1,$2)
           ON CONFLICT DO NOTHING"#,
        user_id, role_id
    )
    .execute(db)
    .await?;
    Ok(())
}
