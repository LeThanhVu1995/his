use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::ProcRepo;
use crate::domain::service::RisService;
use crate::dto::procedure_dto::{CreateProcedureReq, UpdateProcedureReq, ProcQuery, ProcedureRes};
use crate::dto::common::calc_etag;

pub async fn list_procedures(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ProcQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = ProcRepo { db: &db };
    let (items, total) = repo.search_paged(q.q.as_deref(), q.modality.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<ProcedureRes> = items.into_iter().map(|p| ProcedureRes {
        id: p.id,
        code: p.code,
        name: p.name,
        modality: p.modality,
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

pub async fn create_procedure(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateProcedureReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = RisService {
        procs: ProcRepo { db: &db },
        orders: crate::domain::repo::OrderRepo { db: &db },
        studies: crate::domain::repo::StudyRepo { db: &db },
        reports: crate::domain::repo::ReportRepo { db: &db },
    };
    let id = svc.create_procedure(&payload)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create proc");
            crate::error::AppError::Internal("DB".into())
        })?;

    let _repo = ProcRepo { db: &db };
    let p = sqlx::query_as::<_, crate::domain::models::Procedure>(
        r#"SELECT id,code,name,modality,body_part,contrast,duration_min,created_at,updated_at FROM rad_procedures WHERE id=$1"#
    )
    .bind(id)
    .fetch_optional(&**db)
    .await
    .map_err(|_| crate::error::AppError::Internal("DB".into()))?
    .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Created().json(ProcedureRes {
        id: p.id,
        code: p.code,
        name: p.name,
        modality: p.modality,
    }))
}

pub async fn update_procedure(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateProcedureReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = ProcRepo { db: &db }
        .update(path.into_inner(), payload.name.as_deref(), payload.body_part.as_deref(), payload.contrast, payload.duration_min)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ProcedureRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        modality: rec.modality,
    }))
}
