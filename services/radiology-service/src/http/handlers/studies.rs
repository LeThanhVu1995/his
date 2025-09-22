use actix_web::{get, post, put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::StudyRepo;
use crate::domain::service::RisService;
use crate::dto::study_dto::{CreateStudyReq, ProgressStudyReq, StudyQuery, StudyRes};

pub async fn list_studies(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<StudyQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = StudyRepo { db: &db };
    let (items, total) = repo.list_paged(q.order_id, q.status.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<StudyRes> = items.into_iter().map(|s| StudyRes {
        id: s.id,
        accession_no: s.accession_no,
        status: s.status,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

pub async fn create_study(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateStudyReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = RisService {
        procs: crate::domain::repo::ProcRepo { db: &db },
        orders: crate::domain::repo::OrderRepo { db: &db },
        studies: StudyRepo { db: &db },
        reports: crate::domain::repo::ReportRepo { db: &db },
    };
    let id = svc.create_study(payload.order_id, &payload.modality)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create study");
            crate::error::AppError::Internal("DB".into())
        })?;

    let s = StudyRepo { db: &db }
        .find(id)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Created().json(StudyRes {
        id: s.id,
        accession_no: s.accession_no,
        status: s.status,
    }))
}

pub async fn progress_study(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<ProgressStudyReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = StudyRepo { db: &db }
        .set_progress(path.into_inner(), &payload.action, None)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(StudyRes {
        id: rec.id,
        accession_no: rec.accession_no,
        status: rec.status,
    }))
}
