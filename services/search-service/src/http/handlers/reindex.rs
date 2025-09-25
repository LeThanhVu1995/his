use actix_web::{post, web, HttpResponse};
use crate::domain::services::indexer_svc::IndexerSvc;

#[derive(Debug, serde::Deserialize)]
pub struct ReindexReq { pub index_code: String }

#[post("/api/v1/search:reindex")]
pub async fn reindex(db: web::Data<sqlx::Pool<sqlx::Postgres>>, body: web::Json<ReindexReq>) -> actix_web::Result<HttpResponse> {
    let os = crate::infra::opensearch::client::OsClient::from_env();
    let svc = IndexerSvc { db: &db, os };
    let affected = match body.index_code.as_str() {
        "patients" => svc.reindex_patients().await.map_err(|e| {
            tracing::error!(error = ?e, "reindex patients error");
            actix_web::error::ErrorInternalServerError("reindex")
        })?,
        _ => 0,
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true, "affected": affected})))
}
