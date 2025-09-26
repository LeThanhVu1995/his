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
            actix_web::error::ErrorInternalServerError("reindex patients")
        })?,
        "encounters" => svc.reindex_encounters().await.map_err(|e| {
            tracing::error!(error = ?e, "reindex encounters error");
            actix_web::error::ErrorInternalServerError("reindex encounters")
        })?,
        "orders" => svc.reindex_orders().await.map_err(|e| {
            tracing::error!(error = ?e, "reindex orders error");
            actix_web::error::ErrorInternalServerError("reindex orders")
        })?,
        "documents" => svc.reindex_documents().await.map_err(|e| {
            tracing::error!(error = ?e, "reindex documents error");
            actix_web::error::ErrorInternalServerError("reindex documents")
        })?,
        "all" => {
            let patients = svc.reindex_patients().await.map_err(|e| {
                tracing::error!(error = ?e, "reindex patients error");
                actix_web::error::ErrorInternalServerError("reindex patients")
            })?;
            let encounters = svc.reindex_encounters().await.map_err(|e| {
                tracing::error!(error = ?e, "reindex encounters error");
                actix_web::error::ErrorInternalServerError("reindex encounters")
            })?;
            let orders = svc.reindex_orders().await.map_err(|e| {
                tracing::error!(error = ?e, "reindex orders error");
                actix_web::error::ErrorInternalServerError("reindex orders")
            })?;
            let documents = svc.reindex_documents().await.map_err(|e| {
                tracing::error!(error = ?e, "reindex documents error");
                actix_web::error::ErrorInternalServerError("reindex documents")
            })?;
            patients + encounters + orders + documents
        },
        _ => {
            tracing::warn!(index_code = %body.index_code, "unknown index code");
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Unknown index code. Supported: patients, encounters, orders, documents, all"
            })));
        },
    };
    Ok(HttpResponse::Ok().json(serde_json::json!({"ok": true, "affected": affected})))
}
