use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_payer_svc::InsPayerSvc;

pub async fn get_ins_payer(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let payer_id = path.into_inner();
    let svc = InsPayerSvc::new(&db);

    let payer = svc.get_payer_by_id(&payer_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match payer {
        Some(p) => {
            let response = crate::http::dto::ins_payer_dto::InsPayerResponse::from_entity(&p);
            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Payer not found"
        })))
    }
}
