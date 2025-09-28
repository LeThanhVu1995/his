use actix_web::{web, HttpResponse, Result};
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

use crate::domain::services::patient_service::PatientService;
use crate::infra::db::repositories::PatientRepo;
use crate::http::dto::patient::*;
use crate::http::dto::common::ApiResponse;

// Patient CRUD handlers
pub async fn create_patient(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreatePatientRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient = service.create_patient(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(PatientResponse::from_entity(patient))))
}

pub async fn get_patient(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient_id = path.into_inner();
    match service.get_patient(&patient_id).await {
        Ok(Some(patient)) => Ok(HttpResponse::Ok().json(ApiResponse::success(PatientResponse::from_entity(patient)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Patient not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_patient_by_code(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let code = path.into_inner();
    match service.get_patient_by_code(&code).await {
        Ok(Some(patient)) => Ok(HttpResponse::Ok().json(ApiResponse::success(PatientResponse::from_entity(patient)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Patient not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patients(
    db: web::Data<Pool<Postgres>>,
    query: web::Query<ListRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let (patients, total) = service.list_patients(
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<PatientResponse> = patients.into_iter().map(PatientResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_patient(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdatePatientRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient_id = path.into_inner();
    let updated_patient = service.update_patient(&patient_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(PatientResponse::from_entity(updated_patient))))
}

pub async fn delete_patient(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_patient(&patient_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Patient deleted successfully")))
}

// Patient Identifier handlers
pub async fn create_patient_identifier(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreatePatientIdentifierRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let identifier = service.create_patient_identifier(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(PatientIdentifierResponse::from_entity(identifier))))
}

pub async fn get_patient_identifiers(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient_id = path.into_inner();
    let identifiers = service.get_patient_identifiers(&patient_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<PatientIdentifierResponse> = identifiers.into_iter().map(PatientIdentifierResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

// Patient Contact handlers
pub async fn create_patient_contact(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreatePatientContactRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let contact = service.create_patient_contact(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(PatientContactResponse::from_entity(contact))))
}

pub async fn get_patient_contacts(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient_id = path.into_inner();
    let contacts = service.get_patient_contacts(&patient_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<PatientContactResponse> = contacts.into_iter().map(PatientContactResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

// Episode of Care handlers
pub async fn create_episode(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateEpisodeRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let episode = service.create_episode(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(EpisodeResponse::from_entity(episode))))
}

pub async fn get_patient_episodes(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let patient_id = path.into_inner();
    let episodes = service.get_patient_episodes(&patient_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<EpisodeResponse> = episodes.into_iter().map(EpisodeResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn close_episode(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<CloseEpisodeRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let episode_id = path.into_inner();
    let episode = service.close_episode(&episode_id, req.into_inner().end_date).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(EpisodeResponse::from_entity(episode))))
}

// Search handlers
pub async fn search_patients(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<SearchPatientRequest>,
) -> Result<HttpResponse> {
    let service = PatientService::new(PatientRepo { db: &db });
    let (patients, total) = service.search_patients(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<PatientResponse> = patients.into_iter().map(PatientResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}
