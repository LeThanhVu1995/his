use actix_web::{web, HttpResponse};
use crate::domain::services::report_svc::ReportSvc;
use crate::http::dto::query_dto::RangeQuery;

pub async fn overview(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<RangeQuery>,
) -> actix_web::Result<HttpResponse> {
    let data = ReportSvc { db: &db }
        .dashboard_overview(q.start, q.end)
        .await
        .map_err(|_| crate::error::error())?;
    Ok(HttpResponse::Ok().json(data))
}
