use actix_web::{web, HttpResponse};
use uuid::Uuid;
use actix_web_validator::{Query, Json};
use actix_web::web::Path;

use crate::{domain::{repo::EncounterRepo, service::EncounterService}, dto::encounter_dto::{CreateEncounterReq, UpdateEncounterReq, EncounterRes, EncounterQuery}};

#[utoipa::path(
    get,
    path="/api/v1/encounters",
    params(("patient_id"=Option<Uuid>, Query, description="Filter by patient ID"),("status"=Option<String>, Query, description="Filter by status"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth"=[]))
)]
pub async fn list_encounters(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<EncounterQuery>
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = EncounterRepo { db: &db };
    let (items, total) = repo.list_paged(q.patient_id, q.status.as_deref(), page, size).await.map_err(|e| {
        tracing::error!(?e,"list");
        crate::error::AppError::Internal("DB".into())
    })?;
    let res: Vec<EncounterRes> = items.into_iter().map(|e| EncounterRes {
        id: e.id,
        encounter_no: e.encounter_no,
        encounter_type: e.encounter_type,
        status: e.status
    }).collect();
    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

#[utoipa::path(
    post,
    path="/api/v1/encounters",
    request_body=CreateEncounterReq,
    security(("bearer_auth"=[]))
)]
pub async fn create_encounter(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    kafka: web::Data<Option<crate::infrastructure::kafka::Kafka>>,
    payload: Json<CreateEncounterReq>
) -> actix_web::Result<HttpResponse> {
    let repo = EncounterRepo { db: &db };
    let svc = EncounterService { repo, kafka: kafka.as_ref().as_ref() };
    let id = svc.create(&payload).await.map_err(|e| {
        tracing::error!(?e,"create");
        crate::error::AppError::Internal("DB".into())
    })?;
    let repo = EncounterRepo { db: &db };
    let rec = repo.find(id).await.map_err(|e| {
        tracing::error!(?e,"find");
        crate::error::AppError::Internal("DB".into())
    })?.ok_or(crate::error::AppError::NotFound)?;
    let res = EncounterRes {
        id: rec.id,
        encounter_no: rec.encounter_no,
        encounter_type: rec.encounter_type,
        status: rec.status
    };
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    put,
    path="/api/v1/encounters/{id}",
    request_body=UpdateEncounterReq,
    security(("bearer_auth"=[]))
)]
pub async fn update_encounter(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: Path<Uuid>,
    payload: Json<UpdateEncounterReq>
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = EncounterRepo { db: &db };
    let rec = repo.update(id, payload.status.as_deref(), payload.department_code.as_deref(), payload.attending_doctor_id.as_deref())
        .await.map_err(|e| {
            tracing::error!(?e,"update");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = EncounterRes {
        id: rec.id,
        encounter_no: rec.encounter_no,
        encounter_type: rec.encounter_type,
        status: rec.status
    };
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    put,
    path="/api/v1/encounters/{id}:close",
    security(("bearer_auth"=[]))
)]
pub async fn close_encounter(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: Path<Uuid>
) -> actix_web::Result<HttpResponse> {
    let repo = EncounterRepo { db: &db };
    let rec = repo.close(path.into_inner()).await.map_err(|e| {
        tracing::error!(?e,"close");
        crate::error::AppError::Internal("DB".into())
    })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = EncounterRes {
        id: rec.id,
        encounter_no: rec.encounter_no,
        encounter_type: rec.encounter_type,
        status: rec.status
    };
    Ok(HttpResponse::Ok().json(res))
}
