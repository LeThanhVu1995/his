use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::http::dto::invoice_dto::{CreateInvoiceReq, InvoiceRes};
use crate::http::handlers::common::create_billing_service;

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
    let billing_service = create_billing_service(&db);

    let invoice_id = billing_service.create_invoice_from_charges(
        payload.encounter_id,
        payload.patient_id,
        "VND".to_string(),
        None, // due_date
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "create invoice from charges");
        crate::error::AppError::Internal("DB".into())
    })?;

    let invoice = billing_service.get_invoice(invoice_id).await
        .map_err(|e| {
            tracing::error!(?e, "get invoice");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = InvoiceRes {
        id: invoice.invoice_id,
        invoice_no: format!("INV-{}", &invoice.invoice_id.to_string()[..8]),
        total: invoice.total_amount,
        status: invoice.status,
    };

    Ok(HttpResponse::Created().json(res))
}
