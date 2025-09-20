use actix_web::{web, HttpResponse, put};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::infra::db::repositories::invoice_repo;
use crate::http::dto::invoice_dto::InvoiceRes;

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

    let invoice = invoice_repo::update_status(&db, id, "ISSUED")
        .await
        .map_err(|e| {
            tracing::error!(?e, "issue invoice");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = InvoiceRes {
        id: invoice.id,
        invoice_no: invoice.invoice_no,
        total: invoice.total,
        status: invoice.status,
    };

    Ok(HttpResponse::Ok().json(res))
}

