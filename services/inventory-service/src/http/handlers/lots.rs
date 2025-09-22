use actix_web::{web, HttpResponse};
use actix_web_validator::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::LotRepo;
use crate::dto::lot_dto::{CreateLotReq, LotQuery, LotRes};

// #[utoipa::path(
//     get,
//     path = "/api/v1/inv/lots",
//     params(
//         ("item_id" = Option<uuid::Uuid>, Query),
//         ("q" = Option<String>, Query),
//         ("page" = Option<i64>, Query),
//         ("page_size" = Option<i64>, Query)
//     ),
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn list_lots(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<LotQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let repo = LotRepo { db: &db };
    let (items, total) = repo.list_paged(q.item_id, q.q.as_deref(), page, size).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<LotRes> = items.into_iter().map(|l| LotRes {
        id: l.id,
        item_id: l.item_id,
        lot_no: l.lot_no,
        exp_date: l.exp_date,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/lots:create",
//     request_body = CreateLotReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn create_lot(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateLotReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let l = crate::domain::models::Lot {
        id,
        item_id: payload.item_id,
        lot_no: payload.lot_no.clone(),
        exp_date: payload.exp_date,
    };

    LotRepo { db: &db }.create(&l).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(LotRes {
        id,
        item_id: l.item_id,
        lot_no: l.lot_no,
        exp_date: l.exp_date,
    }))
}
