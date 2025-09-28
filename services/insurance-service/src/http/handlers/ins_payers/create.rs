use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_payer_svc::InsPayerSvc;
use crate::http::dto::ins_payer_dto::CreateInsPayerRequest;

pub async fn create_ins_payer(
    db: web::Data<sqlx::PgPool>,
    body: web::Json<CreateInsPayerRequest>,
) -> Result<HttpResponse> {
    let svc = InsPayerSvc::new(&db);

    let payer = svc.create_payer(&body.code, &body.name).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = crate::http::dto::ins_payer_dto::InsPayerResponse::from_entity(&payer);

    Ok(HttpResponse::Created().json(response))
}
