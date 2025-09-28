use actix_web::{web, HttpResponse, Result};
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

use crate::domain::services::encounter_service::EncounterService;
use crate::infra::db::repositories::{EncounterRepo, PatientRepo};
use crate::http::dto::encounter::*;
use crate::http::dto::common::ApiResponse;

// Encounter CRUD handlers
pub async fn create_encounter(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateEncounterRequest>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let encounter = service.create_encounter(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(EncounterResponse::from_entity(encounter))))
}

pub async fn get_encounter(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let encounter_id = path.into_inner();
    match service.get_encounter(&encounter_id).await {
        Ok(Some(encounter)) => Ok(HttpResponse::Ok().json(ApiResponse::success(EncounterResponse::from_entity(encounter)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Encounter not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_encounters(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListEncounterQuery>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let patient_id = path.into_inner();
    let (encounters, total) = service.list_patient_encounters(
        &patient_id,
        query.status.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<EncounterResponse> = encounters.into_iter().map(EncounterResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn list_encounters_by_facility(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListEncounterQuery>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let facility_id = path.into_inner();
    let (encounters, total) = service.list_encounters_by_facility(
        &facility_id,
        query.status.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<EncounterResponse> = encounters.into_iter().map(EncounterResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_encounter(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateEncounterRequest>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let encounter_id = path.into_inner();
    let updated_encounter = service.update_encounter(&encounter_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(EncounterResponse::from_entity(updated_encounter))))
}

pub async fn end_encounter(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let encounter_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.end_encounter(&encounter_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Encounter ended successfully")))
}

// Clinical Note handlers
pub async fn create_clinical_note(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateClinicalNoteRequest>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let note = service.create_clinical_note(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(ClinicalNoteResponse::from_entity(note))))
}

pub async fn get_encounter_notes(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListClinicalNoteQuery>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let encounter_id = path.into_inner();
    let (notes, total) = service.list_encounter_notes(
        &encounter_id,
        query.note_type.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<ClinicalNoteResponse> = notes.into_iter().map(ClinicalNoteResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_clinical_note(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateClinicalNoteRequest>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let note_id = path.into_inner();
    let updated_note = service.update_clinical_note(&note_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(ClinicalNoteResponse::from_entity(updated_note))))
}

pub async fn delete_clinical_note(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = EncounterService::new(EncounterRepo { db: &db }, PatientRepo { db: &db });
    let note_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_clinical_note(&note_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Clinical note deleted successfully")))
}
