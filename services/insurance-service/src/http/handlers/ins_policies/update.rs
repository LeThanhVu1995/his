use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_policy_svc::InsPolicySvc;
use crate::http::dto::ins_policy_dto::UpdateInsPolicyRequest;

pub async fn update_ins_policy(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
    body: web::Json<UpdateInsPolicyRequest>,
) -> Result<HttpResponse> {
    let policy_id = path.into_inner();
    let svc = InsPolicySvc::new(&db);

    // Get existing policy
    let mut policy = svc.get_policy_by_id(&policy_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Policy not found"))?;

    // Update fields
    if let Some(patient_id) = &body.patient_id {
        policy.patient_id = patient_id.clone();
    }
    if let Some(payer_id) = &body.payer_id {
        policy.payer_id = payer_id.clone();
    }
    if let Some(policy_no) = &body.policy_no {
        policy.policy_no = policy_no.clone();
    }
    if let Some(coverage_json) = &body.coverage_json {
        policy.coverage_json = Some(coverage_json.clone());
    }
    if let Some(valid_from) = body.valid_from {
        policy.valid_from = Some(valid_from);
    }
    if let Some(valid_to) = body.valid_to {
        policy.valid_to = Some(valid_to);
    }
    if let Some(status) = &body.status {
        policy.status = status.clone();
    }

    svc.update_policy(&policy).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = crate::http::dto::ins_policy_dto::InsPolicyResponse::from_entity(&policy);

    Ok(HttpResponse::Ok().json(response))
}
