use actix_web::{web, HttpResponse};
use actix_web_validator::{Query, Json};
use crate::domain::repo::{MovementRepo, StockRepo};
use crate::domain::service::InventoryService;
use crate::dto::movement_dto::{MovementQuery, ReceiveReq, IssueReq, TransferReq, AdjustReq, MovementRes};
use crate::security::auth_user::AuthUser;

// #[utoipa::path(
//     get,
//     path = "/api/v1/inv/movements",
//     params(
//         ("mv_type" = Option<String>, Query),
//         ("page" = Option<i64>, Query),
//         ("page_size" = Option<i64>, Query)
//     ),
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn list_movements(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<MovementQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let repo = MovementRepo { db: &db };
    let (items, total) = repo.list_paged(q.mv_type.as_deref(), page, size).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<MovementRes> = items.into_iter().map(|m| MovementRes {
        id: m.id,
        mv_no: m.mv_no,
        mv_type: m.mv_type,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/movements:receive",
//     request_body = ReceiveReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn receive_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<ReceiveReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = InventoryService {
        stocks: StockRepo { db: &db },
        movements: MovementRepo { db: &db },
        db: &db,
    };

    let id = svc.receive_stocks(
        payload.dst_wh,
        payload.lines.clone(),
        payload.note.clone(),
        Some(user.0.sub.as_str()),
    ).await
        .map_err(|e| {
            tracing::warn!(?e, "receive fail");
            crate::error::AppError::Internal("receive failed".into())
        })?;

    Ok(HttpResponse::Created().json(MovementRes {
        id,
        mv_no: format!("REC-{}", &id.to_string()[..8]),
        mv_type: "RECEIVE".into(),
    }))
}

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/movements:issue",
//     request_body = IssueReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn issue_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<IssueReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = InventoryService {
        stocks: StockRepo { db: &db },
        movements: MovementRepo { db: &db },
        db: &db,
    };

    let id = svc.issue_stocks(
        payload.src_wh,
        payload.lines.clone(),
        payload.note.clone(),
        Some(user.0.sub.as_str()),
    ).await
        .map_err(|e| {
            tracing::warn!(?e, "issue fail");
            crate::error::AppError::Internal("issue failed".into())
        })?;

    Ok(HttpResponse::Created().json(MovementRes {
        id,
        mv_no: format!("ISS-{}", &id.to_string()[..8]),
        mv_type: "ISSUE".into(),
    }))
}

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/movements:transfer",
//     request_body = TransferReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn transfer_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<TransferReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = InventoryService {
        stocks: StockRepo { db: &db },
        movements: MovementRepo { db: &db },
        db: &db,
    };

    let id = svc.transfer_stocks(
        payload.src_wh,
        payload.dst_wh,
        payload.lines.clone(),
        payload.note.clone(),
        Some(user.0.sub.as_str()),
    ).await
        .map_err(|e| {
            tracing::warn!(?e, "transfer fail");
            crate::error::AppError::Internal("transfer failed".into())
        })?;

    Ok(HttpResponse::Created().json(MovementRes {
        id,
        mv_no: format!("TRF-{}", &id.to_string()[..8]),
        mv_type: "TRANSFER".into(),
    }))
}

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/movements:adjust",
//     request_body = AdjustReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn adjust_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<AdjustReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = InventoryService {
        stocks: StockRepo { db: &db },
        movements: MovementRepo { db: &db },
        db: &db,
    };

    let id = svc.adjust_stocks(
        payload.wh,
        payload.lines.clone(),
        payload.note.clone(),
        Some(user.0.sub.as_str()),
    ).await
        .map_err(|e| {
            tracing::warn!(?e, "adjust fail");
            crate::error::AppError::Internal("adjust failed".into())
        })?;

    Ok(HttpResponse::Created().json(MovementRes {
        id,
        mv_no: format!("ADJ-{}", &id.to_string()[..8]),
        mv_type: "ADJUST".into(),
    }))
}
