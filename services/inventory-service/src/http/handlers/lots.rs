use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::LotRepo;
use crate::dto::lot_dto::{CreateLotReq, LotQuery, LotRes};

#[utoipa::path(
    get,
    path = "/api/v1/inv/lots",
    params(
        ("item_id" = Option<Uuid>, Query, description = "Filter by item ID"),
        ("q" = Option<String>, Query, description = "Search query"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_lots(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<LotQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (lots, total) = LotRepo { db: &db }
        .list_paged(query.item_id, query.q.as_deref(), page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<LotRes> = lots.into_iter().map(|l| LotRes {
        id: l.id,
        item_id: l.item_id,
        lot_no: l.lot_no,
        exp_date: l.exp_date,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/lots:create",
    request_body = CreateLotReq,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_lot(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateLotReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let lot = crate::domain::models::Lot {
        id,
        item_id: payload.item_id,
        lot_no: payload.lot_no.clone(),
        exp_date: payload.exp_date,
        supplier_id: None,
    };

    LotRepo { db: &db }.create(&lot).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(LotRes {
        id,
        item_id: lot.item_id,
        lot_no: lot.lot_no,
        exp_date: lot.exp_date,
    }))
}
