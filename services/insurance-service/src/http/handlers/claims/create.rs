use actix_web::{web, HttpResponse};
use crate::infrastructure::repositories::claim_repo::ClaimRepo;
use crate::domain::services::claim_svc::ClaimSvc;

pub async fn create(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::claim_dto::CreateClaimReq>,
) -> actix_web::Result<HttpResponse> {
    let items = body.items.iter()
        .map(|i| (i.code.clone(), i.qty, i.unit_price, i.description.clone()))
        .collect();

    let id = ClaimSvc {
        repo: ClaimRepo { db: &db },
        db: &db,
    }
    .create_with_items(body.patient_id, body.member_id, &body.payer, items, body.encounter_id)
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(crate::dto::claim_dto::ClaimRes {
        id,
        claim_no: format!("CLM-{}", &id.to_string()[..8]),
        status: "CREATED".into(),
    }))
}
