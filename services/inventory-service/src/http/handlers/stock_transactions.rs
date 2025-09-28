use actix_web::{web, HttpResponse};
use actix_web::web::Query;
use uuid::Uuid;
use crate::dto::stock_transaction_dto::StockTransactionQuery;

#[utoipa::path(
    get,
    path = "/api/v1/inv/stock-transactions",
    params(
        ("warehouse_id" = Option<Uuid>, Query, description = "Filter by warehouse ID"),
        ("item_id" = Option<Uuid>, Query, description = "Filter by item ID"),
        ("reason_code" = Option<String>, Query, description = "Filter by reason code"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_stock_transactions(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<StockTransactionQuery>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement StockTransactionRepo
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "List stock transactions endpoint - to be implemented",
        "data": []
    })))
}

#[utoipa::path(
    get,
    path = "/api/v1/inv/stock-transactions/{id}",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_stock_transaction(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement StockTransactionRepo
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Get stock transaction endpoint - to be implemented"
    })))
}
