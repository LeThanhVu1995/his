use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::{ScheduleRepo, SlotRepo};
use crate::domain::service::ApptService;
use crate::dto::slot_dto::{SlotQuery, GenerateSlotsReq, SlotRes};
use crate::dto::common::calc_etag;

pub async fn list_slots(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<SlotQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(200);
    let repo = SlotRepo { db: &db };
    let (items, total) = repo.list_range(q.provider_id, q.date_from, q.date_to, q.only_free.unwrap_or(false), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<SlotRes> = items.into_iter().map(|s| SlotRes {
        id: s.id,
        provider_id: s.provider_id,
        room_id: s.room_id,
        starts_at: s.starts_at,
        ends_at: s.ends_at,
        reserved: s.reserved,
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

pub async fn generate_slots(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<GenerateSlotsReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = ApptService {
        schedules: ScheduleRepo { db: &db },
        slots: SlotRepo { db: &db },
        appts: crate::domain::repo::ApptRepo { db: &db },
        db: &db,
    };
    let n = svc.generate_slots(payload.provider_id, payload.date_from, payload.date_to)
        .await
        .map_err(|e| {
            tracing::error!(?e, "generate slots");
            crate::error::AppError::Internal("DB".into())
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "generated": n
    })))
}
