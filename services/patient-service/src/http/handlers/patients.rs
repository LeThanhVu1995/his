use actix_web::{web, HttpResponse};
use uuid::Uuid;
use actix_web_validator::{Query, Json};
use validator::Validate;
use actix_web::web::Path;
use app_web::extractors::auth_user::AuthUser;

use crate::{domain::{repo::PatientRepo, service::PatientService}, dto::patient_dto::{CreatePatientReq, UpdatePatientReq, PatientRes, PatientQuery, calc_etag}};

#[utoipa::path(
    get,
    path="/api/v1/patients",
    params(("q"=Option<String>, Query, description="Search query"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth"=[])),
    responses((status=200, description="List patients", body=[PatientRes]))
)]
pub async fn list_patients(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<PatientQuery>,
    _user: AuthUser
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = PatientRepo { db: &db };
    let (items, total) = repo.list_paged(q.q.as_deref(), page, size).await.map_err(|e| {
        tracing::error!(?e,"list");
        crate::error::AppError::Internal("DB".into())
    })?;
    let res: Vec<PatientRes> = items.into_iter().map(|p| PatientRes {
        id: p.id,
        mrn: p.mrn,
        full_name: p.full_name,
        gender: p.gender,
        birth_date: p.birth_date
    }).collect();

    let body = serde_json::to_vec(&res).unwrap();
    let etag = calc_etag(&body);
    if let Some(tag) = req.headers().get(actix_web::http::header::IF_NONE_MATCH).and_then(|h| h.to_str().ok()) {
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
    path="/api/v1/patients",
    request_body=CreatePatientReq,
    security(("bearer_auth"=[])),
    responses((status=201, description="Created", body=PatientRes))
)]
pub async fn create_patient(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    kafka: web::Data<Option<crate::infrastructure::kafka::Kafka>>,
    payload: Json<CreatePatientReq>
) -> actix_web::Result<HttpResponse> {
    if let Err(e) = payload.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }
    let repo = PatientRepo { db: &db };
    let svc = PatientService { repo, kafka: kafka.as_ref().as_ref() };
    let id = svc.create(&payload).await.map_err(|e| {
        tracing::error!(?e,"create");
        crate::error::AppError::Internal("DB".into())
    })?;
    let repo = PatientRepo { db: &db };
    let rec = repo.find(id).await.map_err(|e| {
        tracing::error!(?e,"get");
        crate::error::AppError::Internal("DB".into())
    })?.ok_or(crate::error::AppError::NotFound)?;
    let res = PatientRes {
        id: rec.id,
        mrn: rec.mrn,
        full_name: rec.full_name,
        gender: rec.gender,
        birth_date: rec.birth_date
    };
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    get,
    path="/api/v1/patients/{id}",
    security(("bearer_auth"=[])),
    responses((status=200, body=PatientRes))
)]
pub async fn get_patient(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: Path<Uuid>
) -> actix_web::Result<HttpResponse> {
    let repo = PatientRepo { db: &db };
    let rec = repo.find(path.into_inner()).await.map_err(|e| {
        tracing::error!(?e,"get");
        crate::error::AppError::Internal("DB".into())
    })?.ok_or(crate::error::AppError::NotFound)?;
    let res = PatientRes {
        id: rec.id,
        mrn: rec.mrn,
        full_name: rec.full_name,
        gender: rec.gender,
        birth_date: rec.birth_date
    };
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    put,
    path="/api/v1/patients/{id}",
    request_body=UpdatePatientReq,
    security(("bearer_auth"=[])),
    responses((status=200, body=PatientRes))
)]
pub async fn update_patient(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: Path<Uuid>,
    payload: Json<UpdatePatientReq>
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = PatientRepo { db: &db };
    let rec = repo.update(id, payload.full_name.as_deref(), payload.gender.as_deref(), payload.birth_date, payload.phone.as_deref(), payload.email.as_deref(), payload.address.as_deref(), payload.is_active)
        .await.map_err(|e| {
            tracing::error!(?e,"update");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = PatientRes {
        id: rec.id,
        mrn: rec.mrn,
        full_name: rec.full_name,
        gender: rec.gender,
        birth_date: rec.birth_date
    };
    Ok(HttpResponse::Ok().json(res))
}
