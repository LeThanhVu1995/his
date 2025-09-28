use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_payer_svc::InsPayerSvc;
use crate::http::dto::ins_payer_dto::UpdateInsPayerRequest;

pub async fn update_ins_payer(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<String>,
    body: web::Json<UpdateInsPayerRequest>,
) -> Result<HttpResponse> {
    let payer_id = path.into_inner();
    let svc = InsPayerSvc::new(&db);

    // Get existing payer
    let mut payer = svc.get_payer_by_id(&payer_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Payer not found"))?;

    // Update fields
    if let Some(code) = &body.code {
        payer.code = code.clone();
    }
    if let Some(name) = &body.name {
        payer.name = name.clone();
    }

    svc.update_payer(&payer).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = crate::http::dto::ins_payer_dto::InsPayerResponse::from_entity(&payer);

    Ok(HttpResponse::Ok().json(response))
}
