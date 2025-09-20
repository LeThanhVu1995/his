use uuid::Uuid;
use crate::infra::db::pool::PgPool;
use crate::domain::entities::role::Role;
use crate::infra::db::repositories::role_repo as repo;

pub async fn list_roles(db: &PgPool) -> Result<Vec<Role>, app_error::AppError> {
    repo::list(db).await
}

pub async fn assign_role(db: &PgPool, user_id: Uuid, role_id: Uuid) -> Result<(), app_error::AppError> {
    repo::assign(db, user_id, role_id).await
}
