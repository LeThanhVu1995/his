use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::store::tasks::TaskStore;
use crate::engine::interpreter::Interpreter;
pub async fn claim(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    user: crate::security::auth_user::AuthUser,
) -> actix_web::Result<HttpResponse> {
    TaskStore { db: &db }
        .claim(path.into_inner(), Uuid::parse_str(&user.0.sub).unwrap())
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn complete(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    let (ins, _step) = TaskStore { db: &db }
        .complete(path.into_inner(), &*body)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Interpreter { db: &db }
        .tick(ins)
        .await
        .map_err(|_| crate::error::AppError::Internal("tick".into()))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn get(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let task = TaskStore { db: &db }
        .get(path.into_inner())
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(task))
}
