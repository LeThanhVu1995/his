use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::ProviderRepo;
use crate::dto::provider_dto::{CreateProviderReq, UpdateProviderReq, ProviderQuery, ProviderRes};
use crate::dto::common::calc_etag;

pub async fn list_providers(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ProviderQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = ProviderRepo { db: &db };
    let (items, total) = repo.search_paged(q.q.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<ProviderRes> = items.into_iter().map(|p| ProviderRes {
        id: p.id,
        code: p.code,
        name: p.name,
        specialty: p.specialty,
    }).collect();

    let body = serde_json::to_vec(&res).unwrap();
    let etag = calc_etag(&body);

    if let Some(tag) = req.headers().get(actix_web::http::header::IF_NONE_MATCH)
        .and_then(|h| h.to_str().ok()) {
        if tag == etag {
            return Ok(HttpResponse::NotModified().finish());
        }
    }

    Ok(HttpResponse::Ok()
        .append_header((actix_web::http::header::ETAG, etag))
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .content_type("application/json")
        .body(body))
}

pub async fn create_provider(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateProviderReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let p = crate::domain::models::Provider {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        specialty: payload.specialty.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    ProviderRepo { db: &db }.create(&p)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(ProviderRes {
        id,
        code: p.code,
        name: p.name,
        specialty: p.specialty,
    }))
}

pub async fn update_provider(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateProviderReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = ProviderRepo { db: &db }
        .update(path.into_inner(), payload.name.as_deref(), payload.specialty.as_deref())
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ProviderRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        specialty: rec.specialty,
    }))
}
