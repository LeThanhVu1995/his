use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::UomRepo;
use crate::dto::uom_dto::{CreateUomDto, UpdateUomDto, UomQuery, UomDto};

#[utoipa::path(
    get,
    path = "/api/v1/inv/uoms",
    params(
        ("q" = Option<String>, Query, description = "Search query"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_uoms(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<UomQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (uoms, total) = UomRepo { db: &db }
        .list_paged(query.q.as_deref(), page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<UomDto> = uoms.into_iter().map(|u| UomDto {
        id: u.id,
        code: u.code,
        name: u.name,
        created_at: u.created_at,
        updated_at: u.updated_at,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/uoms:create",
    request_body = CreateUomDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_uom(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateUomDto>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let uom = crate::domain::models::Uom {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    UomRepo { db: &db }.create(&uom).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(UomDto {
        id,
        code: uom.code,
        name: uom.name,
        created_at: uom.created_at,
        updated_at: uom.updated_at,
    }))
}

#[utoipa::path(
    put,
    path = "/api/v1/inv/uoms/{id}",
    request_body = UpdateUomDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_uom(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateUomDto>,
) -> actix_web::Result<HttpResponse> {
    let rec = UomRepo { db: &db }.update(
        path.into_inner(),
        payload.name.as_deref(),
    ).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(UomDto {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        created_at: rec.created_at,
        updated_at: rec.updated_at,
    }))
}
