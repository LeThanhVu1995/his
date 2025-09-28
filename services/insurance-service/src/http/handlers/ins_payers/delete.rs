use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_payer_svc::InsPayerSvc;

pub async fn delete_ins_payer(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let payer_id = path.into_inner();
    let svc = InsPayerSvc::new(&db);

    svc.delete_payer(&payer_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}
