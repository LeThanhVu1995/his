use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::OrderRepo;
use crate::domain::service::RisService;
use crate::dto::order_dto::{CreateOrderReq, UpdateOrderReq, OrderQuery, OrderRes};
use crate::security::auth_user::AuthUser;
use crate::dto::common::calc_etag;

pub async fn list_orders(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<OrderQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = OrderRepo { db: &db };
    let (items, total) = repo.list_paged(q.patient_id, q.status.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<OrderRes> = items.into_iter().map(|o| OrderRes {
        id: o.id,
        order_no: o.order_no,
        status: o.status,
    }).collect();

    let body = serde_json::to_vec(&res).unwrap();
    let etag = calc_etag(&body);

    if let Some(tag) = req.headers().get(actix_web::http::header::IF_NONE_MATCH)
        .and_then(|h| h.to_str().ok()) {
        if tag == etag {
            return Ok(HttpResponse::NotModified().finish());
        }
    }

    Ok(HttpResponse::Ok()
        .append_header((actix_web::http::header::ETAG, etag))
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .content_type("application/json")
        .body(body))
}

pub async fn create_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateOrderReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = RisService {
        procs: crate::domain::repo::ProcRepo { db: &db },
        orders: OrderRepo { db: &db },
        studies: crate::domain::repo::StudyRepo { db: &db },
        reports: crate::domain::repo::ReportRepo { db: &db },
    };
    let id = svc.create_order(&payload, Some(user.0.sub.as_str()))
        .await
        .map_err(|e| {
            tracing::error!(?e, "create order");
            crate::error::AppError::Internal("DB".into())
        })?;

    let o = OrderRepo { db: &db }
        .find(id)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Created().json(OrderRes {
        id: o.id,
        order_no: o.order_no,
        status: o.status,
    }))
}

pub async fn update_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateOrderReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = OrderRepo { db: &db }
        .update(path.into_inner(), payload.status.as_deref(), payload.scheduled_at)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(OrderRes {
        id: rec.id,
        order_no: rec.order_no,
        status: rec.status,
    }))
}
