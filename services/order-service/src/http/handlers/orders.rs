use actix_web::{web, HttpResponse, HttpRequest};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::dto::order_dto::{CreateOrderReq, UpdateOrderReq, OrderQuery, OrderRes};
use crate::domain::repo::OrderRepo;
use crate::domain::service::OrderService;
use crate::dto::common::calc_etag;

#[utoipa::path(
    get,
    path = "/api/v1/orders",
    params(
        ("patient_id" = Option<uuid::Uuid>, Query,),
        ("encounter_id" = Option<uuid::Uuid>, Query,),
        ("status" = Option<String>, Query,),
        ("page" = Option<i64>, Query,),
        ("page_size" = Option<i64>, Query,)
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_orders(
    req: HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<OrderQuery>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = OrderRepo { db: &db };
    let (items, total) = repo
        .list_paged(q.patient_id, q.encounter_id, q.status.as_deref(), page, size)
        .await
        .map_err(|e| {
            tracing::error!(?e, "list");
            crate::error::AppError::Internal("DB".into())
        })?;

    let res: Vec<OrderRes> = items
        .into_iter()
        .map(|o| OrderRes {
            id: o.id,
            order_no: o.order_no,
            order_type: o.order_type,
            status: o.status,
            priority: o.priority,
        })
        .collect();

    let body = serde_json::to_vec(&res).unwrap();
    let etag = calc_etag(&body);

    if let Some(tag) = req
        .headers()
        .get(actix_web::http::header::IF_NONE_MATCH)
        .and_then(|h| h.to_str().ok())
    {
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

#[utoipa::path(
    post,
    path = "/api/v1/orders:create",
    request_body = CreateOrderReq,
    security(("bearer_auth" = []))
)]
pub async fn create_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateOrderReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    // Validation removed for now

    let svc = OrderService {
        orders: OrderRepo { db: &db },
        items: crate::domain::repo::ItemRepo { db: &db },
    };

    let ordered_by = Some(user.user_id.as_str());
    let id = svc
        .create_order(&payload, ordered_by)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create");
            crate::error::AppError::Internal("DB".into())
        })?;

    let repo = OrderRepo { db: &db };
    let rec = repo
        .find(id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "get");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = OrderRes {
        id: rec.id,
        order_no: rec.order_no,
        order_type: rec.order_type,
        status: rec.status,
        priority: rec.priority,
    };

    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/orders/{id}",
    request_body = UpdateOrderReq,
    security(("bearer_auth" = []))
)]
pub async fn update_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateOrderReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = OrderRepo { db: &db };
    let rec = repo
        .update(
            id,
            payload.priority.as_deref(),
            payload.note.as_deref(),
            payload.status.as_deref(),
        )
        .await
        .map_err(|e| {
            tracing::error!(?e, "update");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = OrderRes {
        id: rec.id,
        order_no: rec.order_no,
        order_type: rec.order_type,
        status: rec.status,
        priority: rec.priority,
    };

    Ok(HttpResponse::Ok().json(res))
}
