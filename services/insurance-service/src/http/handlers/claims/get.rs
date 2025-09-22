use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infrastructure::repositories::claim_repo::ClaimRepo;

pub async fn get(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = ClaimRepo { db: &db };

    let claim = repo.get(id)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(claim))
}
