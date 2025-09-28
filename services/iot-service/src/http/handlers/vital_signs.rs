use actix_web::{web, HttpResponse, Result};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use chrono::Utc;
use crate::domain::entities::vital_sign::{CreateVitalSignRequest, VitalSignResponse, VitalSignItem};
use crate::infra::db::repositories::vital_sign_repo::VitalSignRepo;
use crate::domain::entities::vital_sign::{VitalSignRecord, VitalSignItem as VitalSignItemEntity};

#[utoipa::path(
    post,
    path = "/api/v1/iot/vital-signs:create",
    request_body = CreateVitalSignRequest,
    responses(
        (status = 201, description = "Vital signs created successfully", body = VitalSignResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_vital_signs(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreateVitalSignRequest>,
) -> Result<HttpResponse> {
    let vs_id = Uuid::new_v4();
    let measured_at = body.measured_at.unwrap_or_else(|| Utc::now());

    let record = VitalSignRecord {
        vs_id,
        encounter_id: body.encounter_id,
        patient_id: body.patient_id,
        device_id: body.device_id,
        measured_at,
        recorder_staff_id: body.recorder_staff_id,
        note: body.note.clone(),
    };

    let mut tx = db.begin().await.map_err(|_| actix_web::error::ErrorInternalServerError("DB transaction failed"))?;

    // Create vital sign record
    sqlx::query(
        r#"
        INSERT INTO vital_sign_record (vs_id, encounter_id, patient_id, device_id, measured_at, recorder_staff_id, note)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(&record.vs_id)
    .bind(&record.encounter_id)
    .bind(&record.patient_id)
    .bind(&record.device_id)
    .bind(&record.measured_at)
    .bind(&record.recorder_staff_id)
    .bind(&record.note)
    .execute(&mut *tx)
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create vital sign record"))?;

    // Create vital sign items
    let mut items = Vec::new();
    for item_req in &body.items {
        let item_id = Uuid::new_v4();
        let item = VitalSignItemEntity {
            vs_item_id: item_id,
            vs_id,
            code: item_req.code.clone(),
            value_num: item_req.value_num,
            value_text: item_req.value_text.clone(),
            unit: item_req.unit.clone(),
        };

        sqlx::query(
            r#"
            INSERT INTO vital_sign_item (vs_item_id, vs_id, code, value_num, value_text, unit)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(&item.vs_item_id)
        .bind(&item.vs_id)
        .bind(&item.code)
        .bind(&item.value_num)
        .bind(&item.value_text)
        .bind(&item.unit)
        .execute(&mut *tx)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create vital sign item"))?;

        items.push(VitalSignItem {
            vs_item_id: item.vs_item_id,
            vs_id: item.vs_id,
            code: item.code,
            value_num: item.value_num,
            value_text: item.value_text,
            unit: item.unit,
        });
    }

    tx.commit().await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to commit transaction"))?;

    let response = VitalSignResponse {
        vs_id: record.vs_id,
        encounter_id: record.encounter_id,
        patient_id: record.patient_id,
        device_id: record.device_id,
        measured_at: record.measured_at,
        recorder_staff_id: record.recorder_staff_id,
        note: record.note,
        items,
    };

    Ok(HttpResponse::Created().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/vital-signs/{id}",
    params(
        ("id" = Uuid, Path, description = "Vital sign record ID")
    ),
    responses(
        (status = 200, description = "Vital signs found", body = VitalSignResponse),
        (status = 404, description = "Vital signs not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_vital_signs(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let vs_id = path.into_inner();

    let record = VitalSignRepo { db: &db }
        .get_by_id(vs_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Vital signs not found"))?;

    let items = VitalSignRepo { db: &db }
        .get_items_by_vs_id(vs_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    let response = VitalSignResponse {
        vs_id: record.vs_id,
        encounter_id: record.encounter_id,
        patient_id: record.patient_id,
        device_id: record.device_id,
        measured_at: record.measured_at,
        recorder_staff_id: record.recorder_staff_id,
        note: record.note,
        items: items.into_iter().map(|item| VitalSignItem {
            vs_item_id: item.vs_item_id,
            vs_id: item.vs_id,
            code: item.code,
            value_num: item.value_num,
            value_text: item.value_text,
            unit: item.unit,
        }).collect(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/iot/vital-signs",
    params(
        ("patient_id" = Option<Uuid>, Query, description = "Filter by patient ID"),
        ("encounter_id" = Option<Uuid>, Query, description = "Filter by encounter ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Vital signs list", body = Vec<VitalSignResponse>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_vital_signs(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<VitalSignQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let records = if let Some(patient_id) = query.patient_id {
        VitalSignRepo { db: &db }
            .list_by_patient(patient_id, page_size, offset)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    } else if let Some(encounter_id) = query.encounter_id {
        VitalSignRepo { db: &db }
            .list_by_encounter(encounter_id, page_size, offset)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    } else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Either patient_id or encounter_id must be provided"
        })));
    };

    let mut responses = Vec::new();
    for record in records {
        let items = VitalSignRepo { db: &db }
            .get_items_by_vs_id(record.vs_id)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

        responses.push(VitalSignResponse {
            vs_id: record.vs_id,
            encounter_id: record.encounter_id,
            patient_id: record.patient_id,
            device_id: record.device_id,
            measured_at: record.measured_at,
            recorder_staff_id: record.recorder_staff_id,
            note: record.note,
            items: items.into_iter().map(|item| VitalSignItem {
                vs_item_id: item.vs_item_id,
                vs_id: item.vs_id,
                code: item.code,
                value_num: item.value_num,
                value_text: item.value_text,
                unit: item.unit,
            }).collect(),
        });
    }

    Ok(HttpResponse::Ok().json(responses))
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema, validator::Validate)]
pub struct VitalSignQuery {
    pub patient_id: Option<Uuid>,
    pub encounter_id: Option<Uuid>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
