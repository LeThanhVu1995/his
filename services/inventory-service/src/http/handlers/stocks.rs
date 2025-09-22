use actix_web::{web, HttpResponse};
use actix_web_validator::Query;
use crate::domain::repo::StockRepo;
use crate::dto::stock_dto::{StockRes, StockQuery};

// #[utoipa::path(
//     get,
//     path = "/api/v1/inv/stocks",
//     params(
//         ("warehouse_id" = Option<uuid::Uuid>, Query),
//         ("item_id" = Option<uuid::Uuid>, Query),
//         ("page" = Option<i64>, Query),
//         ("page_size" = Option<i64>, Query)
//     ),
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn list_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<StockQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let repo = StockRepo { db: &db };
    let (items, total) = repo.list_paged(q.warehouse_id, q.item_id, page, size).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<StockRes> = items.into_iter().map(|s| StockRes {
        warehouse_id: s.warehouse_id,
        item_id: s.item_id,
        lot_id: s.lot_id,
        qty: s.qty,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

