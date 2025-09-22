use actix_web::{web, HttpResponse};
use crate::infrastructure::repositories::recon_repo::ReconRepo;
use crate::domain::services::recon_svc::ReconSvc;

pub async fn create(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::recon_dto::CreateReconReq>,
) -> actix_web::Result<HttpResponse> {
    let id = ReconSvc {
        repo: ReconRepo { db: &db },
    }
    .create_batch(
        &body.payer,
        body.period_start,
        body.period_end,
        body.total_claims,
        body.total_amount,
        body.approved_amount,
    )
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": id,
        "batch_no": format!("RCN-{}", &id.to_string()[..8])
    })))
}
