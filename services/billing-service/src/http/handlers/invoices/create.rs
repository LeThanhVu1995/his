use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::domain::services::billing_svc;
use crate::http::dto::invoice_dto::{CreateInvoiceReq, InvoiceRes};
use crate::infra::db::repositories::invoice_repo;

#[post("/api/v1/invoices:create")]
#[utoipa::path(
    post,
    path = "/api/v1/invoices:create",
    request_body = CreateInvoiceReq,
    security(("bearer_auth" = []))
)]
pub async fn create_invoice(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateInvoiceReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = billing_svc::generate_invoice(
        &db,
        payload.patient_id,
        payload.encounter_id,
        payload.charge_ids.clone(),
        payload.discount.clone(),
        payload.tax.clone(),
        payload.note.clone(),
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "generate invoice");
        crate::error::AppError::Internal("DB".into())
    })?;

    let invoice = invoice_repo::find_by_id(&db, id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find invoice");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = InvoiceRes {
        id: invoice.id,
        invoice_no: invoice.invoice_no,
        total: invoice.total,
        status: invoice.status,
    };

    Ok(HttpResponse::Created().json(res))
}
