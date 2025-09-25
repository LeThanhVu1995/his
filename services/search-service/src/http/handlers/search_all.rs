use actix_web::{get, web, HttpResponse};
use crate::domain::services::search_svc::SearchSvc;

#[derive(Debug, serde::Deserialize)]
pub struct SearchQ { pub q: String, pub size: Option<i64> }

#[get("/api/v1/search")]
pub async fn search(db: web::Data<sqlx::Pool<sqlx::Postgres>>, q: web::Query<SearchQ>) -> actix_web::Result<HttpResponse> {
    let _ = db; // currently unused in simple flow
    let os = crate::infra::opensearch::client::OsClient::from_env();
    let svc = SearchSvc::new(os);
    let res = svc.search_all(&q.q, q.size.unwrap_or(20)).await.map_err(|e| {
        tracing::error!(error = ?e, "search error");
        actix_web::error::ErrorInternalServerError("search")
    })?;
    Ok(HttpResponse::Ok().json(res))
}
