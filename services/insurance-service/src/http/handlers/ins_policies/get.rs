use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_policy_svc::InsPolicySvc;

pub async fn get_ins_policy(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let policy_id = path.into_inner();
    let svc = InsPolicySvc::new(&db);

    let policy = svc.get_policy_by_id(&policy_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match policy {
        Some(p) => {
            let response = crate::http::dto::ins_policy_dto::InsPolicyResponse::from_entity(&p);
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Policy not found"
        })))
    }
}
