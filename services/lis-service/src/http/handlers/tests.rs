use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repositories::TestRepo;
use crate::domain::service::LabService;
use crate::http::dto::test_dto::{CreateTestReq, UpdateTestReq, TestQuery, LabTestRes};
use crate::http::dto::common::calc_etag;

#[utoipa::path(
    get,
    path = "/api/v1/lab/tests",
    params(
        ("q" = Option<String>, Query, description = "Search query"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_tests(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<TestQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = TestRepo { db: &db };
    let (items, total) = repo.search_paged(q.q.as_deref(), page, size).await.map_err(|e| {
        tracing::error!(?e, "list tests");
        crate::error::AppError::Internal("DB".into())
    })?;
    let res: Vec<LabTestRes> = items.into_iter().map(|t| LabTestRes {
        id: t.id,
        code: t.code,
        name: t.name,
        specimen_type: t.specimen_type,
        unit: t.unit,
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
    path = "/api/v1/lab/tests:create",
    request_body = CreateTestReq,
    security(("bearer_auth" = []))
)]
pub async fn create_test(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateTestReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = LabService {
        tests: TestRepo { db: &db },
        specimens: crate::domain::repositories::SpecimenRepo { db: &db },
        results: crate::domain::repositories::ResultRepo { db: &db },
    };
    let id = svc.create_test(&payload).await.map_err(|e| {
        tracing::error!(?e, "create test");
        crate::error::AppError::Internal("DB".into())
    })?;
    let repo = TestRepo { db: &db };
    let t = repo.find(id).await.map_err(|e| {
        tracing::error!(?e, "find test");
        crate::error::AppError::Internal("DB".into())
    })?.ok_or(crate::error::AppError::NotFound)?;
    let res = LabTestRes {
        id: t.id,
        code: t.code,
        name: t.name,
        specimen_type: t.specimen_type,
        unit: t.unit,
    };
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/tests/{id}",
    request_body = UpdateTestReq,
    security(("bearer_auth" = []))
)]
pub async fn update_test(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateTestReq>,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = TestRepo { db: &db };
    let rec = repo.update(id, payload.name.as_deref(), payload.unit.as_deref(), payload.ref_low, payload.ref_high)
        .await.map_err(|e| {
            tracing::error!(?e, "update test");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = LabTestRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        specimen_type: rec.specimen_type,
        unit: rec.unit,
    };
    Ok(HttpResponse::Ok().json(res))
}
