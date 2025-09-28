use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::ItemRepo;
use crate::dto::item_dto::{CreateItemReq, UpdateItemReq, ItemQuery, ItemRes};

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
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<ItemQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (items, total) = ItemRepo { db: &db }
        .search_paged(query.q.as_deref(), page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<ItemRes> = items.into_iter().map(|i| ItemRes {
        id: i.id,
        code: i.code,
        name: i.name,
        uom: i.uom,
        is_med: i.is_med,
        is_consumable: i.is_consumable,
    }).collect();

    Ok(HttpResponse::Ok().json(response))
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
    let item = crate::domain::models::Item {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        uom: payload.uom.clone(),
        base_uom_id: None,
        category_code: None,
        is_med: payload.is_med.unwrap_or(false),
        is_consumable: payload.is_consumable.unwrap_or(false),
        is_lot_tracked: true,
        is_expirable: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    ItemRepo { db: &db }.create(&item).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(ItemRes {
        id,
        code: item.code,
        name: item.name,
        uom: item.uom,
        is_med: item.is_med,
        is_consumable: item.is_consumable,
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
