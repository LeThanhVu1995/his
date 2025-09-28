use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use crate::domain::repo::MovementRepo;
use crate::dto::movement_dto::{MovementQuery, MovementRes, ReceiveReq, IssueReq, TransferReq, AdjustReq};

#[utoipa::path(
    get,
    path = "/api/v1/inv/movements",
    params(
        ("mv_type" = Option<String>, Query, description = "Filter by movement type"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_movements(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<MovementQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (movements, total) = MovementRepo { db: &db }
        .list_paged(query.mv_type.as_deref(), page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<MovementRes> = movements.into_iter().map(|m| MovementRes {
        id: m.id,
        mv_no: m.mv_no,
        mv_type: m.mv_type,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/movements:receive",
    request_body = ReceiveReq,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn receive_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<ReceiveReq>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement receive stocks logic using InventoryService
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Receive stocks endpoint - to be implemented"
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/movements:issue",
    request_body = IssueReq,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn issue_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<IssueReq>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement issue stocks logic using InventoryService
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Issue stocks endpoint - to be implemented"
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/movements:transfer",
    request_body = TransferReq,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn transfer_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<TransferReq>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement transfer stocks logic using InventoryService
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Transfer stocks endpoint - to be implemented"
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/movements:adjust",
    request_body = AdjustReq,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn adjust_stocks(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<AdjustReq>,
) -> actix_web::Result<HttpResponse> {
    // TODO: Implement adjust stocks logic using InventoryService
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Adjust stocks endpoint - to be implemented"
    })))
}
