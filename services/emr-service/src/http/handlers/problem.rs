use actix_web::{web, HttpResponse, Result};
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

use crate::domain::services::problem_service::ProblemService;
use crate::infra::db::repositories::ProblemRepo;
use crate::http::dto::problem::*;
use crate::http::dto::common::ApiResponse;

// Problem List handlers
pub async fn create_problem(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateProblemRequest>,
) -> Result<HttpResponse> {
    let service = ProblemService::new(ProblemRepo { db: &db });
    let problem = service.create_problem(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(ProblemResponse::from_entity(problem))))
}

pub async fn get_problem(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = ProblemService::new(ProblemRepo { db: &db });
    let problem_id = path.into_inner();
    match service.get_problem(&problem_id).await {
        Ok(Some(problem)) => Ok(HttpResponse::Ok().json(ApiResponse::success(ProblemResponse::from_entity(problem)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Problem not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_problems(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListProblemQuery>,
) -> Result<HttpResponse> {
    let service = ProblemService::new(ProblemRepo { db: &db });
    let patient_id = path.into_inner();
    let (problems, total) = service.list_patient_problems(
        &patient_id,
        query.status.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<ProblemResponse> = problems.into_iter().map(ProblemResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_problem(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateProblemRequest>,
) -> Result<HttpResponse> {
    let service = ProblemService::new(ProblemRepo { db: &db });
    let problem_id = path.into_inner();
    let updated_problem = service.update_problem(&problem_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(ProblemResponse::from_entity(updated_problem))))
}

pub async fn resolve_problem(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<ResolveProblemRequest>,
) -> Result<HttpResponse> {
    let service = ProblemService::new(ProblemRepo { db: &db });
    let problem_id = path.into_inner();
    // TODO: Get user_id from auth context
    let resolved_problem = service.resolve_problem(&problem_id, req.into_inner().abatement_date, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(ProblemResponse::from_entity(resolved_problem))))
}

pub async fn delete_problem(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = ProblemService::new(ProblemRepo { db: &db });
    let problem_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_problem(&problem_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Problem deleted successfully")))
}
