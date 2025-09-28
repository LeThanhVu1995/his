use actix_web::{web, HttpResponse, get};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::http::dto::invoice_dto::InvoiceRes;
use crate::http::handlers::common::create_billing_service;

#[get("/api/v1/invoices/{id}")]
#[utoipa::path(
    get,
    path = "/api/v1/invoices/{id}",
    security(("bearer_auth" = []))
)]
pub async fn get_invoice(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let invoice_id = path.into_inner();
    let billing_service = create_billing_service(&db);

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

    Ok(HttpResponse::Ok().json(res))
}
