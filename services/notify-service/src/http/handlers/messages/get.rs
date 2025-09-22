use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infrastructure::repositories::message_repo::MessageRepo;

pub async fn get(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let m = MessageRepo { db: &db }.get(path.into_inner()).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(m))
}
