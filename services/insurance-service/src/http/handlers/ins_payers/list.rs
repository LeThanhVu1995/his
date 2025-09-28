use actix_web::{web, HttpResponse, Result};
use crate::domain::services::ins_payer_svc::InsPayerSvc;
use crate::http::dto::ins_payer_dto::ListInsPayersRequest;

pub async fn list_ins_payers(
    db: web::Data<sqlx::PgPool>,
    query: web::Query<ListInsPayersRequest>,
) -> Result<HttpResponse> {
    let svc = InsPayerSvc::new(&db);

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let (payers, total) = tokio::try_join!(
        svc.list_payers(limit, offset),
        svc.count_payers()
    ).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let response = crate::http::dto::ins_payer_dto::ListInsPayersResponse {
        payers: payers.into_iter()
            .map(|p| crate::http::dto::ins_payer_dto::InsPayerResponse::from_entity(&p))
            .collect(),
        total,
    };

    Ok(HttpResponse::Ok().json(response))
}
