use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repositories::ResultRepo;
use crate::domain::service::LabService;
use crate::http::dto::result_dto::{CreateResultReq, EnterResultReq, ResultQuery, LabResultRes};

#[utoipa::path(
    get,
    path = "/api/v1/lab/results",
    params(
        ("specimen_id" = Option<uuid::Uuid>, Query, description = "Specimen ID"),
        ("status" = Option<String>, Query, description = "Result status"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_results(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ResultQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = ResultRepo { db: &db };
    let (items, total) = repo.list_paged(q.specimen_id, q.status.as_deref(), page, size).await.map_err(|e| {
        tracing::error!(?e, "list results");
        crate::error::AppError::Internal("DB".into())
    })?;
    let res: Vec<LabResultRes> = items.into_iter().map(|r| LabResultRes {
        id: r.id,
        result_no: r.result_no,
        status: r.status,
    }).collect();
    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/lab/results:create",
    request_body = CreateResultReq,
    security(("bearer_auth" = []))
)]
pub async fn create_result(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateResultReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = LabService {
        tests: crate::domain::repositories::TestRepo { db: &db },
        specimens: crate::domain::repositories::SpecimenRepo { db: &db },
        results: ResultRepo { db: &db },
    };
    let id = svc.create_result(payload.specimen_id, payload.test_id, payload.note.as_deref()).await.map_err(|e| {
        tracing::error!(?e, "create result");
        crate::error::AppError::Internal("DB".into())
    })?;
    let repo = ResultRepo { db: &db };
    let r = repo.find(id).await.map_err(|e| {
        tracing::error!(?e, "find result");
        crate::error::AppError::Internal("DB".into())
    })?.ok_or(crate::error::AppError::NotFound)?;
    let res = LabResultRes {
        id: r.id,
        result_no: r.result_no,
        status: r.status,
    };
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    post,
    path = "/api/v1/lab/results/{id}:enter",
    request_body = EnterResultReq,
    security(("bearer_auth" = []))
)]
pub async fn enter_values(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<EnterResultReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = LabService {
        tests: crate::domain::repositories::TestRepo { db: &db },
        specimens: crate::domain::repositories::SpecimenRepo { db: &db },
        results: ResultRepo { db: &db },
    };
    svc.enter_values(path.into_inner(), &payload.values).await.map_err(|e| {
        tracing::error!(?e, "enter values");
        crate::error::AppError::Internal("DB".into())
    })?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/results/{id}:verify",
    security(("bearer_auth" = []))
)]
pub async fn verify_result(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let repo = ResultRepo { db: &db };
    let rec = repo.update_status(path.into_inner(), "VERIFIED", None)
        .await.map_err(|e| {
            tracing::error!(?e, "verify");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = LabResultRes {
        id: rec.id,
        result_no: rec.result_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/results/{id}:release",
    security(("bearer_auth" = []))
)]
pub async fn release_result(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let repo = ResultRepo { db: &db };
    let rec = repo.update_status(path.into_inner(), "RELEASED", None)
        .await.map_err(|e| {
            tracing::error!(?e, "release");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = LabResultRes {
        id: rec.id,
        result_no: rec.result_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}
