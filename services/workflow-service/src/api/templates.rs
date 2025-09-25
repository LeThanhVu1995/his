use actix_web::{web, HttpResponse};
use crate::store::templates::TemplateStore;
pub async fn upsert(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    let code = body["code"].as_str().unwrap();
    let name = body["name"].as_str().unwrap();
    let version = body["version"].as_i64().unwrap_or(1) as i32;
    let spec = body["spec"].clone();

    let id = TemplateStore { db: &db }
        .upsert(code, name, version, &spec)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": id})))
}

pub async fn get(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let spec = TemplateStore { db: &db }
        .get(&path)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(spec))
}

pub async fn list(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> actix_web::Result<HttpResponse> {
    let templates = TemplateStore { db: &db }
        .list()
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Ok().json(templates))
}
