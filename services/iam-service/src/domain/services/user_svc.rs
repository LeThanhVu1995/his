use uuid::Uuid;
use sqlx::{PgPool, Postgres, Transaction};
use crate::infra::db::pool::PgPool as PoolAlias;
use crate::infra::db::repositories::user_repo as repo;
use crate::domain::entities::user::User;
// use app_outbox::{enqueue, NewOutboxMsg};  // Disabled due to CMake requirement
use serde_json::json;

pub async fn create_user(db: &PoolAlias, username: &str, full_name: Option<&str>, email: Option<&str>) -> Result<User, app_error::AppError> {
    repo::create(db, username, full_name, email).await
}

/// Create user + enqueue event in the same transaction (disabled due to CMake requirement)
// pub async fn create_user_with_event(db: &PgPool, username: &str, full_name: Option<&str>, email: Option<&str>) -> Result<User, app_error::AppError> {
//     let mut tx: Transaction<'_, Postgres> = db.begin().await?;
//     let user = repo::create(db, username, full_name, email).await?;
//     let msg = NewOutboxMsg::new(
//         "iam.user", &user.id.to_string(), "created",
//         crate::infra::kafka::topics::IAM_EVENTS,
//         json!({
//             "id": user.id,
//             "username": user.username,
//             "full_name": user.full_name,
//             "email": user.email
//         })
//     );
//     enqueue(&mut tx, &msg).await?;
//     tx.commit().await?;
//     Ok(user)
// }

pub async fn get_user(db: &PoolAlias, id: Uuid) -> Result<User, app_error::AppError> {
    repo::get_by_id(db, id).await
}

pub async fn list_users(db: &PoolAlias, offset: i64, limit: i64) -> Result<(Vec<User>, i64), app_error::AppError> {
    repo::list(db, offset, limit).await
}

pub async fn update_user(db: &PoolAlias, id: Uuid, full_name: Option<String>, email: Option<String>, locked: Option<bool>) -> Result<User, app_error::AppError> {
    let u = repo::update(db, id, full_name, email, locked).await?;
    // Event publishing disabled due to CMake requirement
    Ok(u)
}

pub async fn set_lock(db: &PoolAlias, id: Uuid, locked: bool) -> Result<User, app_error::AppError> {
    let u = repo::update(db, id, None, None, Some(locked)).await?;
    // Event publishing disabled due to CMake requirement
    Ok(u)
}
// iam-service src/domain/services/user_svc.rs placeholder
