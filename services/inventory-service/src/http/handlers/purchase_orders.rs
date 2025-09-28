use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::dto::purchase_order_dto::{CreatePurchaseOrderDto, UpdatePurchaseOrderDto, PurchaseOrderQuery};

#[utoipa::path(
    get,
    path = "/api/v1/inv/purchase-orders",
    params(
        ("q" = Option<String>, Query, description = "Search query"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_purchase_orders(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<PurchaseOrderQuery>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement PurchaseOrderRepo
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "List purchase orders endpoint - to be implemented",
        "data": []
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/purchase-orders:create",
    request_body = CreatePurchaseOrderDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_purchase_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreatePurchaseOrderDto>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement PurchaseOrderRepo
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Create purchase order endpoint - to be implemented"
    })))
}

#[utoipa::path(
    put,
    path = "/api/v1/inv/purchase-orders/{id}",
    request_body = UpdatePurchaseOrderDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_purchase_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdatePurchaseOrderDto>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement PurchaseOrderRepo
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Update purchase order endpoint - to be implemented"
    })))
}
