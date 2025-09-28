use actix_web::{web, HttpResponse, Result};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::device_reading::{CreateDeviceReadingRequest, DeviceReadingResponse};
use crate::infra::db::repositories::device_reading_repo::DeviceReadingRepo;

#[utoipa::path(
    post,
    path = "/api/v1/iot/device-readings:create",
    request_body = CreateDeviceReadingRequest,
    responses(
        (status = 201, description = "Device reading created successfully", body = DeviceReadingResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_device_reading(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreateDeviceReadingRequest>,
) -> Result<HttpResponse> {
    let reading_id = Uuid::new_v4();
    let read_at = body.read_at.unwrap_or_else(|| Utc::now());
    let quality = body.quality.clone().unwrap_or_else(|| "GOOD".to_string());

    let reading = crate::domain::entities::device_reading::DeviceReading {
        reading_id,
        device_id: body.device_id,
        sensor_type: body.sensor_type.clone(),
        value_num: body.value_num,
        value_text: body.value_text.clone(),
        unit: body.unit.clone(),
        quality,
        read_at,
        raw_data: body.raw_data.clone(),
    };

    DeviceReadingRepo { db: &db }
        .create(&reading)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create device reading"))?;

    let response = DeviceReadingResponse {
        reading_id: reading.reading_id,
        device_id: reading.device_id,
        sensor_type: reading.sensor_type,
        value_num: reading.value_num,
        value_text: reading.value_text,
        unit: reading.unit,
        quality: reading.quality,
        read_at: reading.read_at,
        raw_data: reading.raw_data,
    };

    Ok(HttpResponse::Created().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/device-readings/{id}",
    params(
        ("id" = Uuid, Path, description = "Device reading ID")
    ),
    responses(
        (status = 200, description = "Device reading found", body = DeviceReadingResponse),
        (status = 404, description = "Device reading not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_device_reading(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let reading_id = path.into_inner();

    let reading = DeviceReadingRepo { db: &db }
        .get_by_id(reading_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Device reading not found"))?;

    let response = DeviceReadingResponse {
        reading_id: reading.reading_id,
        device_id: reading.device_id,
        sensor_type: reading.sensor_type,
        value_num: reading.value_num,
        value_text: reading.value_text,
        unit: reading.unit,
        quality: reading.quality,
        read_at: reading.read_at,
        raw_data: reading.raw_data,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/device-readings",
    params(
        ("device_id" = Option<Uuid>, Query, description = "Filter by device ID"),
        ("sensor_type" = Option<String>, Query, description = "Filter by sensor type"),
        ("hours" = Option<i64>, Query, description = "Filter by recent hours"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Device readings list", body = Vec<DeviceReadingResponse>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_device_readings(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<DeviceReadingQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let readings = if let Some(device_id) = query.device_id {
        if let Some(hours) = query.hours {
            // Get recent readings by hours
            DeviceReadingRepo { db: &db }
                .list_recent_by_device(device_id, hours)
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        } else {
            // Get all readings by device with pagination
            DeviceReadingRepo { db: &db }
                .list_by_device(device_id, page_size, offset)
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        }
    } else if let Some(ref sensor_type) = query.sensor_type {
        DeviceReadingRepo { db: &db }
            .list_by_sensor_type(sensor_type, page_size, offset)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    } else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Either device_id or sensor_type must be provided"
        })));
    };

    let responses: Vec<DeviceReadingResponse> = readings.into_iter().map(|reading| DeviceReadingResponse {
        reading_id: reading.reading_id,
        device_id: reading.device_id,
        sensor_type: reading.sensor_type,
        value_num: reading.value_num,
        value_text: reading.value_text,
        unit: reading.unit,
        quality: reading.quality,
        read_at: reading.read_at,
        raw_data: reading.raw_data,
    }).collect();

    Ok(HttpResponse::Ok().json(responses))
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
pub struct DeviceReadingQuery {
    pub device_id: Option<Uuid>,
    pub sensor_type: Option<String>,
    pub hours: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
