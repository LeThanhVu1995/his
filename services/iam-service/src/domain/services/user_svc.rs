use uuid::Uuid;
use sqlx::{PgPool, Postgres, Transaction};
use crate::infra::db::pool::PgPool as PoolAlias;
use crate::infra::db::repositories::user_repo as repo;
use crate::domain::entities::user::User;
use app_outbox::{enqueue, NewOutboxMsg};
use serde_json::json;

pub async fn create_user(db: &PoolAlias, username: &str, full_name: Option<&str>, email: Option<&str>) -> Result<User, app_error::AppError> {
    repo::create(db, username, full_name, email).await
}

/// Create user + enqueue event in the same transaction
pub async fn create_user_with_event(db: &PgPool, username: &str, full_name: Option<&str>, email: Option<&str>) -> Result<User, app_error::AppError> {
    let mut tx: Transaction<'_, Postgres> = db.begin().await?;

    let user = repo::create(db, id, full_name, email, locked).await?;


    let msg = NewOutboxMsg::new(
        "iam.user", &user.id.to_string(), "created",
        crate::infra::kafka::topics::IAM_EVENTS,
        json!({
            "id": user.id,
            "username": user.username,
            "full_name": user.full_name,
            "email": user.email
        })
    );
    enqueue(&mut tx, &msg).await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn get_user(db: &PoolAlias, id: Uuid) -> Result<User, app_error::AppError> {
    repo::get_by_id(db, id).await
}

pub async fn list_users(db: &PoolAlias, offset: i64, limit: i64) -> Result<(Vec<User>, i64), app_error::AppError> {
    repo::list(db, offset, limit).await
}

pub async fn update_user(db: &PoolAlias, id: Uuid, full_name: Option<String>, email: Option<String>, locked: Option<bool>) -> Result<User, app_error::AppError> {
    let u = repo::update(db, id, full_name, email, locked).await?;

    // fire-and-forget enqueue update event
    let db2 = db.clone();
    let uid = u.id;
    let payload = json!({ "id": u.id, "username": u.username, "full_name": u.full_name, "email": u.email, "locked": u.locked });
    actix_rt::spawn(async move {
        if let Ok(mut tx) = db2.begin().await {
            let _ = enqueue(&mut tx, &NewOutboxMsg::new("iam.user", &uid.to_string(), "updated", crate::infra::kafka::topics::IAM_EVENTS, payload)).await;
            let _ = tx.commit().await;
        }
    });

    Ok(u)
}

pub async fn set_lock(db: &PoolAlias, id: Uuid, locked: bool) -> Result<User, app_error::AppError> {
    let u = repo::update(db, id, None, None, Some(locked)).await?;
    let db2 = db.clone();
    let uid = u.id;
    let payload = serde_json::json!({ "id": u.id, "locked": u.locked });
    actix_rt::spawn(async move {
        if let Ok(mut tx) = db2.begin().await {
            let _ = enqueue(&mut tx, &NewOutboxMsg::new("iam.user", &uid.to_string(), "locked", crate::infra::kafka::topics::IAM_EVENTS, payload)).await;
            let _ = tx.commit().await;
        }
    });
    Ok(u)
}
// iam-service src/domain/services/user_svc.rs placeholder
