use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::{ApptRepo, SlotRepo, ScheduleRepo};
use crate::domain::service::ApptService;
use crate::dto::appt_dto::{ApptQuery, BookApptReq, CancelApptReq, RescheduleApptReq, ApptRes};
use crate::security::auth_user::AuthUser;

pub async fn list_appts(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ApptQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = ApptRepo { db: &db };
    let (items, total) = repo.list_paged(q.patient_id, q.provider_id, q.status.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<ApptRes> = items.into_iter().map(|a| ApptRes {
        id: a.id,
        appt_no: a.appt_no,
        status: a.status,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

pub async fn book_appt(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<BookApptReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = ApptService {
        schedules: ScheduleRepo { db: &db },
        slots: SlotRepo { db: &db },
        appts: ApptRepo { db: &db },
        db: &db,
    };
    let id = svc.book(payload.patient_id, payload.slot_id, payload.reason.clone(), Some(user.0.sub.as_str()))
        .await
        .map_err(|e| {
            tracing::warn!(?e, "book fail");
            crate::error::AppError::Conflict("slot is not available".into())
        })?;

    let a = sqlx::query_as::<_, crate::domain::models::Appointment>(
        r#"SELECT id,appt_no,patient_id,provider_id,room_id,slot_id,status,reason,created_by,created_at,updated_at FROM appointments WHERE id=$1"#
    )
    .bind(id)
    .fetch_one(&**db)
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(ApptRes {
        id: a.id,
        appt_no: a.appt_no,
        status: a.status,
    }))
}

pub async fn cancel_appt(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let svc = ApptService {
        schedules: ScheduleRepo { db: &db },
        slots: SlotRepo { db: &db },
        appts: ApptRepo { db: &db },
        db: &db,
    };
    svc.cancel(path.into_inner())
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn reschedule_appt(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<RescheduleApptReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = ApptService {
        schedules: ScheduleRepo { db: &db },
        slots: SlotRepo { db: &db },
        appts: ApptRepo { db: &db },
        db: &db,
    };
    svc.reschedule(path.into_inner(), payload.new_slot_id, Some(user.0.sub.as_str()))
        .await
        .map_err(|e| {
            tracing::warn!(?e, "reschedule fail");
            crate::error::AppError::Conflict("new slot is not available".into())
        })?;

    Ok(HttpResponse::Ok().finish())
}
