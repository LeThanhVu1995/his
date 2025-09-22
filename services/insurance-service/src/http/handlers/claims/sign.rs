use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infrastructure::repositories::claim_repo::ClaimRepo;
use crate::domain::services::claim_svc::ClaimSvc;

pub async fn sign(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();

    ClaimSvc {
        repo: ClaimRepo { db: &db },
        db: &db,
    }
    .sign(id)
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Ok().finish())
}
