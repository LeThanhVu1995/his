use actix_web::{web, HttpResponse};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::domain::repo::ItemRepo;
use crate::dto::order_dto::{UpdateItemReq, SubmitResultReq, OrderItemRes};

#[utoipa::path(
    get,
    path = "/api/v1/orders/{order_id}/items",
    security(("bearer_auth" = []))
)]
pub async fn list_items(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let repo = ItemRepo { db: &db };
    let items = repo
        .list_by_order(path.into_inner())
        .await
        .map_err(|e| {
            tracing::error!(?e, "items");
            crate::error::AppError::Internal("DB".into())
        })?;

    let res: Vec<OrderItemRes> = items
        .into_iter()
        .map(|i| OrderItemRes {
            id: i.id,
            item_code: i.item_code,
            item_name: i.item_name,
            quantity: i.quantity,
            status: i.status,
        })
        .collect();

    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/order-items/{id}",
    request_body = UpdateItemReq,
    security(("bearer_auth" = []))
)]
pub async fn update_item(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateItemReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = ItemRepo { db: &db };
    let rec = repo
        .update(
            id,
            payload.item_name.as_deref(),
            payload.quantity,
            payload.status.as_deref(),
        )
        .await
        .map_err(|e| {
            tracing::error!(?e, "update item");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = OrderItemRes {
        id: rec.id,
        item_code: rec.item_code,
        item_name: rec.item_name,
        quantity: rec.quantity,
        status: rec.status,
    };

    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/order-items/{id}:result",
    request_body = SubmitResultReq,
    security(("bearer_auth" = []))
)]
pub async fn submit_result(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<SubmitResultReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = ItemRepo { db: &db };
    let rec = repo
        .submit_result(id, &payload.result_json, payload.status.as_deref())
        .await
        .map_err(|e| {
            tracing::error!(?e, "result");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = OrderItemRes {
        id: rec.id,
        item_code: rec.item_code,
        item_name: rec.item_name,
        quantity: rec.quantity,
        status: rec.status,
    };

    Ok(HttpResponse::Ok().json(res))
}
