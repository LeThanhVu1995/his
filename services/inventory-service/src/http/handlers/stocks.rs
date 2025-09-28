use actix_web::{web, HttpResponse};
use actix_web::web::Query;
use crate::domain::repo::StockRepo;
use crate::dto::stock_dto::{StockQuery, StockRes};
use rust_decimal::prelude::ToPrimitive;

#[utoipa::path(
    get,
    path = "/api/v1/inv/stocks",
    params(
        ("warehouse_id" = Option<Uuid>, Query, description = "Filter by warehouse ID"),
        ("item_id" = Option<Uuid>, Query, description = "Filter by item ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<StockQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (stocks, total) = StockRepo { db: &db }
        .list_paged(query.warehouse_id, query.item_id, page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<StockRes> = stocks.into_iter().map(|s| StockRes {
        warehouse_id: s.warehouse_id,
        item_id: s.item_id,
        lot_id: s.lot_id,
        qty: s.qty.to_f64().unwrap_or(0.0),
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .json(response))
}
