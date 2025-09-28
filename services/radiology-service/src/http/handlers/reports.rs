use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::ReportRepo;
use crate::domain::service::RisService;
use crate::dto::report_dto::{CreateReportReq, EditReportReq, ReportQuery, ReportRes};
use crate::security::auth_user::AuthUser;

pub async fn list_reports(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<ReportQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = ReportRepo { db: &db };
    let (items, total) = repo.list_paged(q.study_id, q.status.as_deref(), page, size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let res: Vec<ReportRes> = items.into_iter().map(|r| ReportRes {
        id: r.id,
        report_no: r.report_no,
        status: r.status,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

pub async fn create_report(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: web::Json<CreateReportReq>,
) -> actix_web::Result<HttpResponse> {
    let svc = RisService {
        procs: crate::domain::repo::ProcRepo { db: &db },
        orders: crate::domain::repo::OrderRepo { db: &db },
        studies: crate::domain::repo::StudyRepo { db: &db },
        reports: ReportRepo { db: &db },
    };
    let id = svc.create_report(payload.study_id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create report");
            crate::error::AppError::Internal("DB".into())
        })?;

    let r = ReportRepo { db: &db }
        .find(id)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Created().json(ReportRes {
        id: r.id,
        report_no: r.report_no,
        status: r.status,
    }))
}

pub async fn edit_report(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<EditReportReq>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let rec = ReportRepo { db: &db }
        .set_content(path.into_inner(), &payload.content, &user.0.sub)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ReportRes {
        id: rec.id,
        report_no: rec.report_no,
        status: rec.status,
    }))
}

pub async fn verify_report(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let rec = ReportRepo { db: &db }
        .set_status(path.into_inner(), "PRELIM", Some(&user.0.sub))
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ReportRes {
        id: rec.id,
        report_no: rec.report_no,
        status: rec.status,
    }))
}

pub async fn final_report(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let rec = ReportRepo { db: &db }
        .set_status(path.into_inner(), "FINAL", Some(&user.0.sub))
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ReportRes {
        id: rec.id,
        report_no: rec.report_no,
        status: rec.status,
    }))
}
