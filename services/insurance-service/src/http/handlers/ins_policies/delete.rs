use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_policy_svc::InsPolicySvc;

pub async fn delete_ins_policy(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let policy_id = path.into_inner();
    let svc = InsPolicySvc::new(&db);

    svc.delete_policy(&policy_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}
