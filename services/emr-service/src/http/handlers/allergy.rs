use actix_web::{web, HttpResponse, Result};
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

use crate::domain::services::allergy_service::AllergyService;
use crate::infra::db::repositories::AllergyRepo;
use crate::http::dto::allergy::*;
use crate::http::dto::common::ApiResponse;

// Allergy Intolerance handlers
pub async fn create_allergy(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateAllergyRequest>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let allergy = service.create_allergy(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(AllergyResponse::from_entity(allergy))))
}

pub async fn get_allergy(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let allergy_id = path.into_inner();
    match service.get_allergy(&allergy_id).await {
        Ok(Some(allergy)) => Ok(HttpResponse::Ok().json(ApiResponse::success(AllergyResponse::from_entity(allergy)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Allergy not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_allergies(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListAllergyQuery>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let patient_id = path.into_inner();
    let (allergies, total) = service.list_patient_allergies(
        &patient_id,
        query.category.as_deref(),
        query.severity.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<AllergyResponse> = allergies.into_iter().map(AllergyResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_allergy(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateAllergyRequest>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let allergy_id = path.into_inner();
    let updated_allergy = service.update_allergy(&allergy_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(AllergyResponse::from_entity(updated_allergy))))
}

pub async fn delete_allergy(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let allergy_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_allergy(&allergy_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Allergy deleted successfully")))
}

// Medication Statement handlers
pub async fn create_medication(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateMedicationRequest>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let medication = service.create_medication(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(MedicationResponse::from_entity(medication))))
}

pub async fn get_medication(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let medication_id = path.into_inner();
    match service.get_medication(&medication_id).await {
        Ok(Some(medication)) => Ok(HttpResponse::Ok().json(ApiResponse::success(MedicationResponse::from_entity(medication)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Medication not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_medications(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListMedicationQuery>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let patient_id = path.into_inner();
    let (medications, total) = service.list_patient_medications(
        &patient_id,
        query.status.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<MedicationResponse> = medications.into_iter().map(MedicationResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_medication(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateMedicationRequest>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let medication_id = path.into_inner();
    let updated_medication = service.update_medication(&medication_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(MedicationResponse::from_entity(updated_medication))))
}

pub async fn delete_medication(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = AllergyService::new(AllergyRepo { db: &db });
    let medication_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_medication(&medication_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Medication deleted successfully")))
}
