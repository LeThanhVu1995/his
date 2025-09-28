use actix_web::{web, HttpResponse, Result};
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

use crate::domain::services::order_service::OrderService;
use crate::infra::db::repositories::OrderRepo;
use crate::http::dto::order::*;
use crate::http::dto::common::ApiResponse;

// Clinical Order handlers
pub async fn create_order(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let order = service.create_order(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(OrderResponse::from_entity(order))))
}

pub async fn get_order(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let order_id = path.into_inner();
    match service.get_order(&order_id).await {
        Ok(Some(order)) => Ok(HttpResponse::Ok().json(ApiResponse::success(OrderResponse::from_entity(order)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Order not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_orders(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListOrderQuery>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let patient_id = path.into_inner();
    let (orders, total) = service.list_patient_orders(
        &patient_id,
        query.order_type.as_deref(),
        query.status.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<OrderResponse> = orders.into_iter().map(OrderResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn list_encounter_orders(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListOrderQuery>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let encounter_id = path.into_inner();
    let (orders, total) = service.list_encounter_orders(
        &encounter_id,
        query.order_type.as_deref(),
        query.status.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<OrderResponse> = orders.into_iter().map(OrderResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_order(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateOrderRequest>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let order_id = path.into_inner();
    let updated_order = service.update_order(&order_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(OrderResponse::from_entity(updated_order))))
}

pub async fn complete_order(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<CompleteOrderRequest>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let order_id = path.into_inner();
    // TODO: Get user_id from auth context
        let completed_order = service.complete_order(&order_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(OrderResponse::from_entity(completed_order))))
}

pub async fn cancel_order(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let order_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.cancel_order(&order_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Order cancelled successfully")))
}

pub async fn delete_order(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = OrderService::new(OrderRepo { db: &db });
    let order_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_order(&order_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Order deleted successfully")))
}
