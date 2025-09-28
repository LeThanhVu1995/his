use actix_web::{web, HttpResponse, Result};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::observation::{CreateObservationRequest, UpdateObservationRequest, ObservationResponse};
use crate::infra::db::repositories::observation_repo::ObservationRepo;

#[utoipa::path(
    post,
    path = "/api/v1/iot/observations:create",
    request_body = CreateObservationRequest,
    responses(
        (status = 201, description = "Observation created successfully", body = ObservationResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_observation(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreateObservationRequest>,
) -> Result<HttpResponse> {
    let obs_id = Uuid::new_v4();
    let taken_at = body.taken_at.unwrap_or_else(|| Utc::now());
    let status = body.status.clone().unwrap_or_else(|| "FINAL".to_string());

    let observation = crate::domain::entities::observation::Observation {
        obs_id,
        encounter_id: body.encounter_id,
        patient_id: body.patient_id,
        device_id: body.device_id,
        code: body.code.clone(),
        value_num: body.value_num,
        value_text: body.value_text.clone(),
        unit: body.unit.clone(),
        taken_at,
        performer_staff_id: body.performer_staff_id,
        status,
    };

    ObservationRepo { db: &db }
        .create(&observation)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create observation"))?;

    let response = ObservationResponse {
        obs_id: observation.obs_id,
        encounter_id: observation.encounter_id,
        patient_id: observation.patient_id,
        device_id: observation.device_id,
        code: observation.code,
        value_num: observation.value_num,
        value_text: observation.value_text,
        unit: observation.unit,
        taken_at: observation.taken_at,
        performer_staff_id: observation.performer_staff_id,
        status: observation.status,
    };

    Ok(HttpResponse::Created().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/observations/{id}",
    params(
        ("id" = Uuid, Path, description = "Observation ID")
    ),
    responses(
        (status = 200, description = "Observation found", body = ObservationResponse),
        (status = 404, description = "Observation not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_observation(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let obs_id = path.into_inner();

    let observation = ObservationRepo { db: &db }
        .get_by_id(obs_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Observation not found"))?;

    let response = ObservationResponse {
        obs_id: observation.obs_id,
        encounter_id: observation.encounter_id,
        patient_id: observation.patient_id,
        device_id: observation.device_id,
        code: observation.code,
        value_num: observation.value_num,
        value_text: observation.value_text,
        unit: observation.unit,
        taken_at: observation.taken_at,
        performer_staff_id: observation.performer_staff_id,
        status: observation.status,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    put,
    path = "/api/v1/iot/observations/{id}",
    request_body = UpdateObservationRequest,
    responses(
        (status = 200, description = "Observation updated successfully", body = ObservationResponse),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Observation not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_observation(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: Json<UpdateObservationRequest>,
) -> Result<HttpResponse> {
    let obs_id = path.into_inner();

    let observation = ObservationRepo { db: &db }
        .update(
            obs_id,
            body.value_num,
            body.value_text.clone(),
            body.unit.clone(),
            body.taken_at,
            body.performer_staff_id,
            body.status.clone(),
        )
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Observation not found"))?;

    let response = ObservationResponse {
        obs_id: observation.obs_id,
        encounter_id: observation.encounter_id,
        patient_id: observation.patient_id,
        device_id: observation.device_id,
        code: observation.code,
        value_num: observation.value_num,
        value_text: observation.value_text,
        unit: observation.unit,
        taken_at: observation.taken_at,
        performer_staff_id: observation.performer_staff_id,
        status: observation.status,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/observations",
    params(
        ("patient_id" = Option<Uuid>, Query, description = "Filter by patient ID"),
        ("encounter_id" = Option<Uuid>, Query, description = "Filter by encounter ID"),
        ("code" = Option<String>, Query, description = "Filter by observation code"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Observations list", body = Vec<ObservationResponse>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_observations(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<ObservationQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let observations = if let Some(patient_id) = query.patient_id {
        ObservationRepo { db: &db }
            .list_by_patient(patient_id, page_size, offset)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    } else if let Some(encounter_id) = query.encounter_id {
        ObservationRepo { db: &db }
            .list_by_encounter(encounter_id, page_size, offset)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    } else if let Some(ref code) = query.code {
        ObservationRepo { db: &db }
            .list_by_code(code, page_size, offset)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    } else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "At least one filter parameter must be provided"
        })));
    };

    let responses: Vec<ObservationResponse> = observations.into_iter().map(|obs| ObservationResponse {
        obs_id: obs.obs_id,
        encounter_id: obs.encounter_id,
        patient_id: obs.patient_id,
        device_id: obs.device_id,
        code: obs.code,
        value_num: obs.value_num,
        value_text: obs.value_text,
        unit: obs.unit,
        taken_at: obs.taken_at,
        performer_staff_id: obs.performer_staff_id,
        status: obs.status,
    }).collect();

    Ok(HttpResponse::Ok().json(responses))
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
pub struct ObservationQuery {
    pub patient_id: Option<Uuid>,
    pub encounter_id: Option<Uuid>,
    pub code: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
