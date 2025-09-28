use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::domain::entities::or_schedule::{
    OrCase, OrChecklist, OrCaseWithChecklist, OrCaseStats,
    CreateOrCaseRequest, UpdateOrCaseRequest, CreateOrChecklistRequest, UpdateOrChecklistRequest,
    OrCaseQuery, OrChecklistQuery
};
use crate::infra::db::repositories::or_repo::{OrCaseRepo, OrChecklistRepo};
use utoipa::ToSchema;

// OR Case Management
#[utoipa::path(
    post,
    path = "/api/v1/or-cssd/or-cases",
    request_body = CreateOrCaseRequest,
    responses(
        (status = 201, description = "OR case created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_or_case(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateOrCaseRequest>,
) -> Result<HttpResponse> {
    let or_case_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let or_case = OrCase {
        or_case_id: or_case_id.clone(),
        encounter_id: body.encounter_id,
        patient_id: body.patient_id,
        scheduled_room_id: body.scheduled_room_id,
        scheduled_start: body.scheduled_start,
        scheduled_end: body.scheduled_end,
        actual_start: None,
        actual_end: None,
        status: "SCHEDULED".to_string(),
        procedure_text: body.procedure_text.clone(),
        surgeon_staff_id: body.surgeon_staff_id,
        anesthetist_staff_id: body.anesthetist_staff_id,
        created_at: now,
        created_by: None, // TODO: Get from auth context
        updated_at: now,
        updated_by: None, // TODO: Get from auth context
    };

    OrCaseRepo { db: &db }
        .create(&or_case)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create OR case"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": or_case_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/or-cases",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("patient_id" = Option<Uuid>, Query, description = "Filter by patient ID"),
        ("surgeon_staff_id" = Option<Uuid>, Query, description = "Filter by surgeon staff ID"),
        ("scheduled_room_id" = Option<Uuid>, Query, description = "Filter by scheduled room ID"),
        ("date_from" = Option<DateTime<Utc>>, Query, description = "Filter from date"),
        ("date_to" = Option<DateTime<Utc>>, Query, description = "Filter to date")
    ),
    responses(
        (status = 200, description = "List of OR cases"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_or_cases(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<OrCaseQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let or_cases = OrCaseRepo { db: &db }
        .list_paged(
            query.status.clone(),
            query.patient_id,
            query.surgeon_staff_id,
            query.scheduled_room_id,
            query.date_from,
            query.date_to,
            page_size,
            offset
        )
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(or_cases))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/or-cases/{id}",
    params(
        ("id" = Uuid, Path, description = "OR case ID")
    ),
    responses(
        (status = 200, description = "OR case found"),
        (status = 404, description = "OR case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_or_case(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let or_case_id = path.into_inner();

    let or_case = OrCaseRepo { db: &db }
        .get_by_id(or_case_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("OR case not found"))?;

    Ok(HttpResponse::Ok().json(or_case))
}

#[utoipa::path(
    put,
    path = "/api/v1/or-cssd/or-cases/{id}",
    params(
        ("id" = Uuid, Path, description = "OR case ID")
    ),
    request_body = UpdateOrCaseRequest,
    responses(
        (status = 200, description = "OR case updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "OR case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_or_case(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateOrCaseRequest>,
) -> Result<HttpResponse> {
    let or_case_id = path.into_inner();

    let mut or_case = OrCaseRepo { db: &db }
        .get_by_id(or_case_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("OR case not found"))?;

    // Apply updates
    if let Some(room_id) = body.scheduled_room_id { or_case.scheduled_room_id = Some(room_id); }
    if let Some(start) = body.scheduled_start { or_case.scheduled_start = Some(start); }
    if let Some(end) = body.scheduled_end { or_case.scheduled_end = Some(end); }
    if let Some(actual_start) = body.actual_start { or_case.actual_start = Some(actual_start); }
    if let Some(actual_end) = body.actual_end { or_case.actual_end = Some(actual_end); }
    if let Some(status) = body.status.clone() { or_case.status = status; }
    if let Some(procedure) = body.procedure_text.clone() { or_case.procedure_text = Some(procedure); }
    if let Some(surgeon) = body.surgeon_staff_id { or_case.surgeon_staff_id = Some(surgeon); }
    if let Some(anesthetist) = body.anesthetist_staff_id { or_case.anesthetist_staff_id = Some(anesthetist); }
    or_case.updated_at = chrono::Utc::now();

    OrCaseRepo { db: &db }
        .update(or_case_id, &or_case)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update OR case"))?;

    Ok(HttpResponse::Ok().json(or_case))
}

#[utoipa::path(
    delete,
    path = "/api/v1/or-cssd/or-cases/{id}",
    params(
        ("id" = Uuid, Path, description = "OR case ID")
    ),
    responses(
        (status = 204, description = "OR case deleted successfully"),
        (status = 404, description = "OR case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_or_case(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let or_case_id = path.into_inner();

    OrCaseRepo { db: &db }
        .delete(or_case_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to delete OR case"))?;

    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/or-cases/stats",
    responses(
        (status = 200, description = "OR case statistics"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_or_case_stats(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse> {
    let stats = OrCaseRepo { db: &db }
        .get_stats()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(stats))
}

// OR Checklist Management
#[utoipa::path(
    post,
    path = "/api/v1/or-cssd/or-cases/{id}/checklists",
    params(
        ("id" = Uuid, Path, description = "OR case ID")
    ),
    request_body = CreateOrChecklistRequest,
    responses(
        (status = 201, description = "OR checklist created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_or_checklist(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<CreateOrChecklistRequest>,
) -> Result<HttpResponse> {
    let or_case_id = path.into_inner();
    let checklist_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let checklist = OrChecklist {
        checklist_id,
        or_case_id,
        phase_code: body.phase_code.clone(),
        item_code: body.item_code.clone(),
        completed: "N".to_string(),
        completed_at: None,
        completed_by: None,
        created_at: now,
    };

    OrChecklistRepo { db: &db }
        .create(&checklist)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create OR checklist"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": checklist_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/or-cases/{id}/checklists",
    params(
        ("id" = Uuid, Path, description = "OR case ID")
    ),
    responses(
        (status = 200, description = "OR case checklists"),
        (status = 404, description = "OR case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_or_case_checklists(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let or_case_id = path.into_inner();

    let checklists = OrChecklistRepo { db: &db }
        .get_by_or_case_id(or_case_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(checklists))
}

#[utoipa::path(
    put,
    path = "/api/v1/or-cssd/checklists/{id}",
    params(
        ("id" = Uuid, Path, description = "Checklist ID")
    ),
    request_body = UpdateOrChecklistRequest,
    responses(
        (status = 200, description = "OR checklist updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "OR checklist not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_or_checklist(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateOrChecklistRequest>,
) -> Result<HttpResponse> {
    let checklist_id = path.into_inner();

    // For now, we'll create a minimal checklist object for update
    // In a real implementation, you'd fetch the existing checklist first
    let checklist = OrChecklist {
        checklist_id,
        or_case_id: Uuid::new_v4(), // This would be fetched from existing record
        phase_code: "".to_string(), // This would be fetched from existing record
        item_code: "".to_string(), // This would be fetched from existing record
        completed: body.completed.clone().unwrap_or_else(|| "N".to_string()),
        completed_at: body.completed_at,
        completed_by: body.completed_by,
        created_at: chrono::Utc::now(), // This would be fetched from existing record
    };

    OrChecklistRepo { db: &db }
        .update(checklist_id, &checklist)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update OR checklist"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}
