use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_policy_svc::InsPolicySvc;
use crate::http::dto::ins_policy_dto::ListInsPoliciesRequest;

pub async fn list_ins_policies(
    db: web::Data<sqlx::PgPool>,
    query: web::Query<ListInsPoliciesRequest>,
) -> Result<HttpResponse> {
    let svc = InsPolicySvc::new(&db);

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let (policies, total) = if let Some(patient_id) = &query.patient_id {
        tokio::try_join!(
            svc.list_policies_by_patient(patient_id, limit, offset),
            svc.count_policies_by_patient(patient_id)
        ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    } else if let Some(payer_id) = &query.payer_id {
        tokio::try_join!(
            svc.list_policies_by_payer(payer_id, limit, offset),
            svc.count_policies_by_payer(payer_id)
        ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    } else if let Some(status) = &query.status {
        tokio::try_join!(
            svc.list_policies_by_status(status, limit, offset),
            svc.count_policies_by_status(status)
        ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    } else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "At least one filter parameter is required"
        })));
    };

    let response = crate::http::dto::ins_policy_dto::ListInsPoliciesResponse {
        policies: policies.into_iter()
            .map(|p| crate::http::dto::ins_policy_dto::InsPolicyResponse::from_entity(&p))
            .collect(),
        total,
    };

    Ok(HttpResponse::Ok().json(response))
}
