use actix_web::{web, HttpResponse, Result};
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;

use crate::domain::services::vital_service::VitalService;
use crate::infra::db::repositories::VitalRepo;
use crate::http::dto::vital::*;
use crate::http::dto::common::ApiResponse;

// Vital Sign Record handlers
pub async fn create_vital_sign_record(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateVitalSignRecordRequest>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let record = service.create_vital_sign_record(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(VitalSignRecordResponse::from_entity(record))))
}

pub async fn get_vital_sign_record(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let record_id = path.into_inner();
    match service.get_vital_sign_record(&record_id).await {
        Ok(Some(record)) => Ok(HttpResponse::Ok().json(ApiResponse::success(VitalSignRecordResponse::from_entity(record)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Vital sign record not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_vital_signs(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListVitalSignQuery>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let patient_id = path.into_inner();
    let (records, total) = service.list_patient_vital_signs(
        &patient_id,
        query.record_type.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<VitalSignRecordResponse> = records.into_iter().map(VitalSignRecordResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_vital_sign_record(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateVitalSignRecordRequest>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let record_id = path.into_inner();
    let updated_record = service.update_vital_sign_record(&record_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(VitalSignRecordResponse::from_entity(updated_record))))
}

pub async fn delete_vital_sign_record(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let record_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_vital_sign_record(&record_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Vital sign record deleted successfully")))
}

// Vital Sign Item handlers
pub async fn create_vital_sign_item(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateVitalSignItemRequest>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let item = service.create_vital_sign_item(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(VitalSignItemResponse::from_entity(item))))
}

pub async fn get_vital_sign_items(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let record_id = path.into_inner();
    let items = service.get_vital_sign_items(&record_id).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<VitalSignItemResponse> = items.into_iter().map(VitalSignItemResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_vital_sign_item(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateVitalSignItemRequest>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let item_id = path.into_inner();
    let updated_item = service.update_vital_sign_item(&item_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(VitalSignItemResponse::from_entity(updated_item))))
}

pub async fn delete_vital_sign_item(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let item_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_vital_sign_item(&item_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Vital sign item deleted successfully")))
}

// Observation handlers
pub async fn create_observation(
    db: web::Data<Pool<Postgres>>,
    req: web::Json<CreateObservationRequest>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let observation = service.create_observation(req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Created().json(ApiResponse::success(ObservationResponse::from_entity(observation))))
}

pub async fn get_observation(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let observation_id = path.into_inner();
    match service.get_observation(&observation_id).await {
        Ok(Some(observation)) => Ok(HttpResponse::Ok().json(ApiResponse::success(ObservationResponse::from_entity(observation)))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<String>::error("Observation not found".to_string()))),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn list_patient_observations(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    query: web::Query<ListObservationQuery>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let patient_id = path.into_inner();
    let (observations, total) = service.list_patient_observations(
        &patient_id,
        query.observation_type.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let responses: Vec<ObservationResponse> = observations.into_iter().map(ObservationResponse::from_entity).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::success(responses)))
}

pub async fn update_observation(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
    req: web::Json<UpdateObservationRequest>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let observation_id = path.into_inner();
    let updated_observation = service.update_observation(&observation_id, req.into_inner()).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(ObservationResponse::from_entity(updated_observation))))
}

pub async fn delete_observation(
    db: web::Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let service = VitalService::new(VitalRepo { db: &db });
    let observation_id = path.into_inner();
    // TODO: Get user_id from auth context
    service.delete_observation(&observation_id, "system").await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("Observation deleted successfully")))
}
