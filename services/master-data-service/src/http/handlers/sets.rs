use actix_web::{web, HttpResponse};
use actix_web::web::Json;
use serde::Deserialize;

use crate::domain::repo::MasterRepo;

#[derive(Debug, Deserialize)]
pub struct CreateSetReq {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}

pub async fn list_sets(db: web::Data<sqlx::Pool<sqlx::Postgres>>) -> actix_web::Result<HttpResponse> {
    let repo = MasterRepo { db: &db };
    let items = repo.list_code_sets().await.map_err(|e| { tracing::error!(?e, "list_sets"); crate::error::AppError::Internal("DB".into()) })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn create_set(db: web::Data<sqlx::Pool<sqlx::Postgres>>, payload: Json<CreateSetReq>) -> actix_web::Result<HttpResponse> {
    let repo = MasterRepo { db: &db };
    let created = repo.create_code_set(&payload.code, &payload.name, payload.description.as_deref())
        .await.map_err(|e| { tracing::error!(?e, "create_set"); crate::error::AppError::Internal("DB".into()) })?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn list_codes_in_set(db: web::Data<sqlx::Pool<sqlx::Postgres>>, path: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let code = path.into_inner();
    let repo = MasterRepo { db: &db };
    let items = repo.list_codes_by_set(&code).await.map_err(|e| { tracing::error!(?e, "list_codes_in_set"); crate::error::AppError::Internal("DB".into()) })?;
    Ok(HttpResponse::Ok().json(items))
}

#[derive(Debug, Deserialize)]
pub struct CreateCodeInSetReq {
    pub code: String,
    pub display: String,
    pub extra_json: Option<String>,
}

pub async fn create_code_in_set(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<String>,
    payload: Json<CreateCodeInSetReq>
) -> actix_web::Result<HttpResponse> {
    let set_code = path.into_inner();
    let repo = MasterRepo { db: &db };
    let created = repo.create_code_in_set(&set_code, &payload.code, &payload.display, payload.extra_json.as_deref())
        .await.map_err(|e| { tracing::error!(?e, "create_code_in_set"); crate::error::AppError::Internal("DB".into()) })?;
    Ok(HttpResponse::Created().json(created))
}


