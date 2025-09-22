use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::RoomRepo;
use crate::dto::room_dto::{CreateRoomReq, UpdateRoomReq, RoomQuery, RoomRes};
use crate::dto::common::calc_etag;

pub async fn list_rooms(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<RoomQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = RoomRepo { db: &db };
    let (items, total) = repo.search_paged(q.q.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<RoomRes> = items.into_iter().map(|r| RoomRes {
        id: r.id,
        code: r.code,
        name: r.name,
        location: r.location,
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

pub async fn create_room(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateRoomReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let r = crate::domain::models::Room {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        location: payload.location.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    RoomRepo { db: &db }.create(&r)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(RoomRes {
        id,
        code: r.code,
        name: r.name,
        location: r.location,
    }))
}

pub async fn update_room(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateRoomReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = RoomRepo { db: &db }
        .update(path.into_inner(), payload.name.as_deref(), payload.location.as_deref())
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(RoomRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        location: rec.location,
    }))
}
