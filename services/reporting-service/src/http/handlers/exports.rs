use actix_web::{web, HttpResponse};
use crate::domain::services::export_svc::ExportSvc;
use crate::http::dto::export_dto::ExportRange;

pub async fn revenue_xlsx(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ExportRange>,
) -> actix_web::Result<HttpResponse> {
    let bytes = ExportSvc { db: &db }
        .export_revenue_xlsx(q.start, q.end)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("export"))?;
    Ok(HttpResponse::Ok()
        .append_header((
            "Content-Type",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        ))
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=Revenue_{}-{}.xlsx", q.start, q.end),
        ))
        .body(bytes))
}
