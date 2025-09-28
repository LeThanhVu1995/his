use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::dto::goods_receipt_dto::{CreateGoodsReceiptDto, UpdateGoodsReceiptDto, GoodsReceiptQuery};

#[utoipa::path(
    get,
    path = "/api/v1/inv/goods-receipts",
    params(
        ("q" = Option<String>, Query, description = "Search query"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_goods_receipts(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<GoodsReceiptQuery>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement GoodsReceiptRepo
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "List goods receipts endpoint - to be implemented",
        "data": []
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/goods-receipts:create",
    request_body = CreateGoodsReceiptDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_goods_receipt(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateGoodsReceiptDto>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement GoodsReceiptRepo
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Create goods receipt endpoint - to be implemented"
    })))
}

#[utoipa::path(
    put,
    path = "/api/v1/inv/goods-receipts/{id}",
    request_body = UpdateGoodsReceiptDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_goods_receipt(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateGoodsReceiptDto>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement GoodsReceiptRepo
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Update goods receipt endpoint - to be implemented"
    })))
}
