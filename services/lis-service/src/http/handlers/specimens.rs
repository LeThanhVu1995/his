use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repositories::SpecimenRepo;
use crate::domain::service::LabService;
use crate::http::dto::specimen_dto::{CreateSpecimenReq, SpecimenQuery, SpecimenRes};
use crate::http::dto::common::calc_etag;
use crate::security::auth_user::AuthUser;

#[utoipa::path(
    get,
    path = "/api/v1/lab/specimens",
    params(
        ("patient_id" = Option<uuid::Uuid>, Query, description = "Patient ID"),
        ("status" = Option<String>, Query, description = "Specimen status"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_specimens(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<SpecimenQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = SpecimenRepo { db: &db };
    let (items, total) = repo.list_paged(q.patient_id, q.status.as_deref(), page, size).await.map_err(|e| {
        tracing::error!(?e, "list spm");
        crate::error::AppError::Internal("DB".into())
    })?;
    let res: Vec<SpecimenRes> = items.into_iter().map(|s| SpecimenRes {
        id: s.id,
        specimen_no: s.specimen_no,
        status: s.status,
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
    path = "/api/v1/lab/specimens:create",
    request_body = CreateSpecimenReq,
    security(("bearer_auth" = []))
)]
pub async fn create_specimen(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateSpecimenReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = LabService {
        tests: crate::domain::repositories::TestRepo { db: &db },
        specimens: SpecimenRepo { db: &db },
        results: crate::domain::repositories::ResultRepo { db: &db },
    };
    let id = svc.create_specimen(&payload).await.map_err(|e| {
        tracing::error!(?e, "create spm");
        crate::error::AppError::Internal("DB".into())
    })?;
    let repo = SpecimenRepo { db: &db };
    let s = repo.find(id).await.map_err(|e| {
        tracing::error!(?e, "find spm");
        crate::error::AppError::Internal("DB".into())
    })?.ok_or(crate::error::AppError::NotFound)?;
    let res = SpecimenRes {
        id: s.id,
        specimen_no: s.specimen_no,
        status: s.status,
    };
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/specimens/{id}:collect",
    security(("bearer_auth" = []))
)]
pub async fn collect_specimen(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let repo = SpecimenRepo { db: &db };
    let rec = repo.update_status(path.into_inner(), "COLLECTED", Some(chrono::Utc::now()), Some(user.0.sub.as_str()))
        .await.map_err(|e| {
            tracing::error!(?e, "collect");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = SpecimenRes {
        id: rec.id,
        specimen_no: rec.specimen_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/specimens/{id}:receive",
    security(("bearer_auth" = []))
)]
pub async fn receive_specimen(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let repo = SpecimenRepo { db: &db };
    let rec = repo.update_status(path.into_inner(), "RECEIVED", None, None)
        .await.map_err(|e| {
            tracing::error!(?e, "receive");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = SpecimenRes {
        id: rec.id,
        specimen_no: rec.specimen_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/specimens/{id}:reject",
    security(("bearer_auth" = []))
)]
pub async fn reject_specimen(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let repo = SpecimenRepo { db: &db };
    let rec = repo.update_status(path.into_inner(), "REJECTED", None, None)
        .await.map_err(|e| {
            tracing::error!(?e, "reject");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = SpecimenRes {
        id: rec.id,
        specimen_no: rec.specimen_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}
