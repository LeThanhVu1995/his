use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_policy_svc::InsPolicySvc;
use crate::http::dto::ins_policy_dto::CreateInsPolicyRequest;

pub async fn create_ins_policy(
    db: web::Data<sqlx::PgPool>,
    body: web::Json<CreateInsPolicyRequest>,
) -> Result<HttpResponse> {
    let svc = InsPolicySvc::new(&db);

    let policy = svc.create_policy(
        &body.patient_id,
        &body.payer_id,
        &body.policy_no,
        body.coverage_json.as_deref(),
        body.valid_from,
        body.valid_to,
    ).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = crate::http::dto::ins_policy_dto::InsPolicyResponse::from_entity(&policy);

    Ok(HttpResponse::Created().json(response))
}
