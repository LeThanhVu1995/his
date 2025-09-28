use actix_web::{web, HttpResponse, Result};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::domain::services::ingest_svc::IngestSvc;
use crate::infra::db::repositories::device_repo::DeviceRepo;
use crate::domain::entities::device::{CreateDeviceRequest, UpdateDeviceRequest};

#[utoipa::path(
    post,
    path = "/api/v1/iot/devices:upsert",
    request_body = CreateDeviceRequest,
    responses(
        (status = 201, description = "Device upserted successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn upsert(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreateDeviceRequest>,
) -> Result<HttpResponse> {
    let id = IngestSvc { db: &db }
        .upsert_device(&body.code, &body.name, &body.r#type, body.location.as_deref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to upsert device"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/devices/{id}",
    params(
        ("id" = Uuid, Path, description = "Device ID")
    ),
    responses(
        (status = 200, description = "Device found"),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_device(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let device_id = path.into_inner();

    let device = DeviceRepo { db: &db }
        .get_by_id(device_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Device not found"))?;

    Ok(HttpResponse::Ok().json(device))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/devices",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Devices list"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_devices(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<DeviceQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let devices = DeviceRepo { db: &db }
        .list_paged(page_size, offset)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(devices))
}

#[utoipa::path(
    put,
    path = "/api/v1/iot/devices/{id}",
    request_body = UpdateDeviceRequest,
    responses(
        (status = 200, description = "Device updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_device(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: Json<UpdateDeviceRequest>,
) -> Result<HttpResponse> {
    let device_id = path.into_inner();

    let device = sqlx::query_as::<_, crate::domain::entities::device::Device>(
        r#"
        UPDATE iot_devices
        SET name = COALESCE($2, name),
            type = COALESCE($3, type),
            location = COALESCE($4, location)
        WHERE id = $1
        RETURNING id,code,name,type as "r#type!",location,last_seen,created_at
        "#
    )
    .bind(device_id)
    .bind(body.name.as_deref())
    .bind(body.r#type.as_deref())
    .bind(body.location.as_deref())
    .fetch_optional(&**db)
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    .ok_or_else(|| actix_web::error::ErrorNotFound("Device not found"))?;

    Ok(HttpResponse::Ok().json(device))
}

#[utoipa::path(
    delete,
    path = "/api/v1/iot/devices/{id}",
    responses(
        (status = 204, description = "Device deleted successfully"),
        (status = 404, description = "Device not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_device(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let device_id = path.into_inner();

    let rows_affected = sqlx::query("DELETE FROM iot_devices WHERE id = $1")
        .bind(device_id)
        .execute(&**db)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .rows_affected();

    if rows_affected == 0 {
        return Err(actix_web::error::ErrorNotFound("Device not found"));
    }

    Ok(HttpResponse::NoContent().finish())
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
pub struct DeviceQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
