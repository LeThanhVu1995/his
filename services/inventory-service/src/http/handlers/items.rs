use actix_web::{web, HttpResponse};
use actix_web_validator::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::ItemRepo;
use crate::dto::item_dto::{CreateItemReq, UpdateItemReq, ItemQuery, ItemRes};
use crate::dto::common::calc_etag;

// #[utoipa::path(
//     get,
//     path = "/api/v1/inv/items",
//     params(
//         ("q" = Option<String>, Query),
//         ("page" = Option<i64>, Query),
//         ("page_size" = Option<i64>, Query)
//     ),
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn list_items(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<ItemQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let repo = ItemRepo { db: &db };
    let (items, total) = repo.search_paged(q.q.as_deref(), page, size).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<ItemRes> = items.into_iter().map(|i| ItemRes {
        id: i.id,
        code: i.code,
        name: i.name,
        uom: i.uom,
        is_med: i.is_med,
        is_consumable: i.is_consumable,
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

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/items:create",
//     request_body = CreateItemReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn create_item(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateItemReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let i = crate::domain::models::Item {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        uom: payload.uom.clone(),
        is_med: payload.is_med.unwrap_or(false),
        is_consumable: payload.is_consumable.unwrap_or(true),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    ItemRepo { db: &db }.create(&i).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(ItemRes {
        id,
        code: i.code,
        name: i.name,
        uom: i.uom,
        is_med: i.is_med,
        is_consumable: i.is_consumable,
    }))
}

// #[utoipa::path(
//     put,
//     path = "/api/v1/inv/items/{id}",
//     request_body = UpdateItemReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn update_item(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateItemReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = ItemRepo { db: &db }.update(
        path.into_inner(),
        payload.name.as_deref(),
        payload.uom.as_deref(),
        payload.is_consumable,
    ).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ItemRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        uom: rec.uom,
        is_med: rec.is_med,
        is_consumable: rec.is_consumable,
    }))
}
