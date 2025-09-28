use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::domain::entities::cssd_tray::{
    CssdTray, CssdTrayItem, CssdSterilizationLot, CssdLotItem,
    CssdTrayWithItems, CssdSterilizationLotWithItems, CssdStats,
    CreateCssdTrayRequest, UpdateCssdTrayRequest, CreateCssdTrayItemRequest, UpdateCssdTrayItemRequest,
    CreateCssdSterilizationLotRequest, UpdateCssdSterilizationLotRequest, CreateCssdLotItemRequest,
    CssdTrayQuery, CssdTrayItemQuery, CssdSterilizationLotQuery, CssdLotItemQuery
};
use crate::infra::db::repositories::cssd_repo::{CssdTrayRepo, CssdTrayItemRepo, CssdSterilizationLotRepo, CssdLotItemRepo};
use utoipa::ToSchema;

// CSSD Tray Management
#[utoipa::path(
    post,
    path = "/api/v1/or-cssd/cssd/trays",
    request_body = CreateCssdTrayRequest,
    responses(
        (status = 201, description = "CSSD tray created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_cssd_tray(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateCssdTrayRequest>,
) -> Result<HttpResponse> {
    let tray_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let tray = CssdTray {
        tray_id: tray_id.clone(),
        code: body.code.clone(),
        name: body.name.clone(),
        description: body.description.clone(),
        created_at: now,
        created_by: None, // TODO: Get from auth context
        updated_at: now,
        updated_by: None, // TODO: Get from auth context
    };

    CssdTrayRepo { db: &db }
        .create(&tray)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create CSSD tray"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": tray_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/trays",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("code" = Option<String>, Query, description = "Filter by code"),
        ("name" = Option<String>, Query, description = "Filter by name")
    ),
    responses(
        (status = 200, description = "List of CSSD trays"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_cssd_trays(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<CssdTrayQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let trays = CssdTrayRepo { db: &db }
        .list_paged(query.code.clone(), query.name.clone(), page_size, offset)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(trays))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/trays/{id}",
    params(
        ("id" = Uuid, Path, description = "Tray ID")
    ),
    responses(
        (status = 200, description = "CSSD tray found"),
        (status = 404, description = "CSSD tray not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_cssd_tray(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let tray_id = path.into_inner();

    let tray = CssdTrayRepo { db: &db }
        .get_by_id(tray_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("CSSD tray not found"))?;

    Ok(HttpResponse::Ok().json(tray))
}

#[utoipa::path(
    put,
    path = "/api/v1/or-cssd/cssd/trays/{id}",
    params(
        ("id" = Uuid, Path, description = "Tray ID")
    ),
    request_body = UpdateCssdTrayRequest,
    responses(
        (status = 200, description = "CSSD tray updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "CSSD tray not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_cssd_tray(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateCssdTrayRequest>,
) -> Result<HttpResponse> {
    let tray_id = path.into_inner();

    let mut tray = CssdTrayRepo { db: &db }
        .get_by_id(tray_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("CSSD tray not found"))?;

    // Apply updates
    if let Some(name) = body.name.clone() { tray.name = name; }
    if let Some(description) = body.description.clone() { tray.description = Some(description); }
    tray.updated_at = chrono::Utc::now();

    CssdTrayRepo { db: &db }
        .update(tray_id, &tray)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update CSSD tray"))?;

    Ok(HttpResponse::Ok().json(tray))
}

#[utoipa::path(
    delete,
    path = "/api/v1/or-cssd/cssd/trays/{id}",
    params(
        ("id" = Uuid, Path, description = "Tray ID")
    ),
    responses(
        (status = 204, description = "CSSD tray deleted successfully"),
        (status = 404, description = "CSSD tray not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_cssd_tray(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let tray_id = path.into_inner();

    CssdTrayRepo { db: &db }
        .delete(tray_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to delete CSSD tray"))?;

    Ok(HttpResponse::NoContent().finish())
}

// CSSD Tray Items Management
#[utoipa::path(
    post,
    path = "/api/v1/or-cssd/cssd/trays/{id}/items",
    params(
        ("id" = Uuid, Path, description = "Tray ID")
    ),
    request_body = CreateCssdTrayItemRequest,
    responses(
        (status = 201, description = "CSSD tray item created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_cssd_tray_item(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<CreateCssdTrayItemRequest>,
) -> Result<HttpResponse> {
    let tray_id = path.into_inner();
    let item_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let item = CssdTrayItem {
        tray_item_id: item_id.clone(),
        tray_id,
        instrument_code: body.instrument_code.clone(),
        quantity: body.quantity,
        created_at: now,
    };

    CssdTrayItemRepo { db: &db }
        .create(&item)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create CSSD tray item"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": item_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/trays/{id}/items",
    params(
        ("id" = Uuid, Path, description = "Tray ID")
    ),
    responses(
        (status = 200, description = "CSSD tray items"),
        (status = 404, description = "CSSD tray not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_cssd_tray_items(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let tray_id = path.into_inner();

    let items = CssdTrayItemRepo { db: &db }
        .get_by_tray_id(tray_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(items))
}

// CSSD Sterilization Lot Management
#[utoipa::path(
    post,
    path = "/api/v1/or-cssd/cssd/sterilization-lots",
    request_body = CreateCssdSterilizationLotRequest,
    responses(
        (status = 201, description = "CSSD sterilization lot created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_cssd_sterilization_lot(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateCssdSterilizationLotRequest>,
) -> Result<HttpResponse> {
    let lot_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let lot = CssdSterilizationLot {
        lot_id: lot_id.clone(),
        lot_code: body.lot_code.clone(),
        method_code: body.method_code.clone(),
        started_at: body.started_at.unwrap_or(now),
        completed_at: None,
        released_by: None,
        status: "IN_PROGRESS".to_string(),
        created_at: now,
    };

    CssdSterilizationLotRepo { db: &db }
        .create(&lot)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create CSSD sterilization lot"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": lot_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/sterilization-lots",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("lot_code" = Option<String>, Query, description = "Filter by lot code"),
        ("method_code" = Option<String>, Query, description = "Filter by method code"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("date_from" = Option<DateTime<Utc>>, Query, description = "Filter from date"),
        ("date_to" = Option<DateTime<Utc>>, Query, description = "Filter to date")
    ),
    responses(
        (status = 200, description = "List of CSSD sterilization lots"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_cssd_sterilization_lots(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<CssdSterilizationLotQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let lots = CssdSterilizationLotRepo { db: &db }
        .list_paged(
            query.lot_code.clone(),
            query.method_code.clone(),
            query.status.clone(),
            query.date_from,
            query.date_to,
            page_size,
            offset
        )
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(lots))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/sterilization-lots/{id}",
    params(
        ("id" = Uuid, Path, description = "Sterilization lot ID")
    ),
    responses(
        (status = 200, description = "CSSD sterilization lot found"),
        (status = 404, description = "CSSD sterilization lot not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_cssd_sterilization_lot(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let lot_id = path.into_inner();

    let lot = CssdSterilizationLotRepo { db: &db }
        .get_by_id(lot_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("CSSD sterilization lot not found"))?;

    Ok(HttpResponse::Ok().json(lot))
}

#[utoipa::path(
    put,
    path = "/api/v1/or-cssd/cssd/sterilization-lots/{id}",
    params(
        ("id" = Uuid, Path, description = "Sterilization lot ID")
    ),
    request_body = UpdateCssdSterilizationLotRequest,
    responses(
        (status = 200, description = "CSSD sterilization lot updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "CSSD sterilization lot not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_cssd_sterilization_lot(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateCssdSterilizationLotRequest>,
) -> Result<HttpResponse> {
    let lot_id = path.into_inner();

    let mut lot = CssdSterilizationLotRepo { db: &db }
        .get_by_id(lot_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("CSSD sterilization lot not found"))?;

    // Apply updates
    if let Some(completed_at) = body.completed_at { lot.completed_at = Some(completed_at); }
    if let Some(released_by) = body.released_by { lot.released_by = Some(released_by); }
    if let Some(status) = body.status.clone() { lot.status = status; }

    CssdSterilizationLotRepo { db: &db }
        .update(lot_id, &lot)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update CSSD sterilization lot"))?;

    Ok(HttpResponse::Ok().json(lot))
}

// CSSD Lot Items Management
#[utoipa::path(
    post,
    path = "/api/v1/or-cssd/cssd/sterilization-lots/{id}/items",
    params(
        ("id" = Uuid, Path, description = "Sterilization lot ID")
    ),
    request_body = CreateCssdLotItemRequest,
    responses(
        (status = 201, description = "CSSD lot item created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_cssd_lot_item(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<CreateCssdLotItemRequest>,
) -> Result<HttpResponse> {
    let lot_id = path.into_inner();
    let item_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let item = CssdLotItem {
        lot_item_id: item_id.clone(),
        lot_id,
        tray_id: body.tray_id,
        expiry_date: body.expiry_date,
        created_at: now,
    };

    CssdLotItemRepo { db: &db }
        .create(&item)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create CSSD lot item"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": item_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/sterilization-lots/{id}/items",
    params(
        ("id" = Uuid, Path, description = "Sterilization lot ID")
    ),
    responses(
        (status = 200, description = "CSSD lot items"),
        (status = 404, description = "CSSD sterilization lot not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_cssd_lot_items(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let lot_id = path.into_inner();

    let items = CssdLotItemRepo { db: &db }
        .get_by_lot_id(lot_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(items))
}

#[utoipa::path(
    get,
    path = "/api/v1/or-cssd/cssd/expired-items",
    responses(
        (status = 200, description = "List of expired CSSD items"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_expired_cssd_items(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse> {
    let items = CssdLotItemRepo { db: &db }
        .get_expired_items()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(items))
}
