use actix_web::{web, HttpResponse, put};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::http::dto::invoice_dto::InvoiceRes;
use crate::http::handlers::common::create_billing_service;

#[put("/api/v1/invoices/{id}:issue")]
#[utoipa::path(
    put,
    path = "/api/v1/invoices/{id}:issue",
    security(("bearer_auth" = []))
)]
pub async fn issue_invoice(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();

    let billing_service = create_billing_service(&db);

    billing_service.update_invoice_status(id, "OPEN").await
        .map_err(|e| {
            tracing::error!(?e, "issue invoice");
            crate::error::AppError::Internal("DB".into())
        })?;

    let invoice = billing_service.get_invoice(id).await
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

    Ok(HttpResponse::Ok().json(res))
}

