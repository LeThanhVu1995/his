use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infrastructure::repositories::claim_repo::ClaimRepo;
use crate::domain::services::claim_svc::ClaimSvc;

pub async fn set_status(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<(Uuid, String)>,
) -> actix_web::Result<HttpResponse> {
    let (id, st) = path.into_inner();

    ClaimSvc {
        repo: ClaimRepo { db: &db },
        db: &db,
    }
    .set_status(id, &st)
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Ok().finish())
}
