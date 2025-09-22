use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::ScheduleRepo;
use crate::domain::models::Schedule;
use crate::dto::schedule_dto::{CreateScheduleReq, UpdateScheduleReq, ScheduleQuery, ScheduleRes};
use crate::dto::common::calc_etag;

pub async fn list_schedules(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ScheduleQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = ScheduleRepo { db: &db };
    let (items, total) = repo.list_paged(q.provider_id, q.weekday, page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<ScheduleRes> = items.into_iter().map(|s| ScheduleRes {
        id: s.id,
        provider_id: s.provider_id,
        room_id: s.room_id,
        weekday: s.weekday,
        start_time: s.start_time,
        end_time: s.end_time,
        slot_min: s.slot_min,
        active: s.active,
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

pub async fn create_schedule(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateScheduleReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let s = Schedule {
        id,
        provider_id: payload.provider_id,
        room_id: payload.room_id,
        weekday: payload.weekday,
        start_time: payload.start_time,
        end_time: payload.end_time,
        slot_min: payload.slot_min,
        active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    ScheduleRepo { db: &db }.create(&s)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(ScheduleRes {
        id,
        provider_id: s.provider_id,
        room_id: s.room_id,
        weekday: s.weekday,
        start_time: s.start_time,
        end_time: s.end_time,
        slot_min: s.slot_min,
        active: s.active,
    }))
}

pub async fn update_schedule(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateScheduleReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = ScheduleRepo { db: &db }
        .update(path.into_inner(), payload.room_id, payload.weekday, payload.start_time, payload.end_time, payload.slot_min, payload.active)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ScheduleRes {
        id: rec.id,
        provider_id: rec.provider_id,
        room_id: rec.room_id,
        weekday: rec.weekday,
        start_time: rec.start_time,
        end_time: rec.end_time,
        slot_min: rec.slot_min,
        active: rec.active,
    }))
}
