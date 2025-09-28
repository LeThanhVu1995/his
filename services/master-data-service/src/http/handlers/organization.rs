use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::domain::models::{
    OrgHospital, OrgFacility, OrgDepartment, OrgRoom, OrgBed,
    CreateHospitalRequest, UpdateHospitalRequest, CreateFacilityRequest, UpdateFacilityRequest,
    CreateDepartmentRequest, UpdateDepartmentRequest, CreateRoomRequest, UpdateRoomRequest,
    CreateBedRequest, UpdateBedRequest,
    HospitalQuery, FacilityQuery, DepartmentQuery, RoomQuery, BedQuery
};
// use crate::infra::db::repositories::organization_repo::{OrgHospitalRepo, OrgFacilityRepo};
use utoipa::ToSchema;

// Hospital Management
#[utoipa::path(
    post,
    path = "/api/v1/master/hospitals",
    request_body = CreateHospitalRequest,
    responses(
        (status = 201, description = "Hospital created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_hospital(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateHospitalRequest>,
) -> Result<HttpResponse> {
    let hospital_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let hospital = OrgHospital {
        hospital_id,
        code: body.code.clone(),
        name: body.name.clone(),
        status: body.status.clone().unwrap_or_else(|| "ACTIVE".to_string()),
        created_at: now,
        created_by: None,
        updated_at: now,
        updated_by: None,
        deleted_at: None,
        deleted_by: None,
    };

    // OrgHospitalRepo { db: &db }
    //     .create(&hospital)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create hospital"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": hospital_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/hospitals",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("code" = Option<String>, Query, description = "Filter by hospital code"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of hospitals"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_hospitals(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<HospitalQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    // let hospitals = OrgHospitalRepo { db: &db }
    //     .list_paged(query.code.clone(), query.status.clone(), page_size, offset)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;
    let hospitals: Vec<OrgHospital> = vec![];

    Ok(HttpResponse::Ok().json(hospitals))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/hospitals/{id}",
    params(
        ("id" = Uuid, Path, description = "Hospital ID")
    ),
    responses(
        (status = 200, description = "Hospital found"),
        (status = 404, description = "Hospital not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_hospital(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let hospital_id = path.into_inner();

    // let hospital = OrgHospitalRepo { db: &db }
    //     .get_by_id(hospital_id)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    //     .ok_or_else(|| actix_web::error::ErrorNotFound("Hospital not found"))?;
    return Err(actix_web::error::ErrorNotFound("Hospital not found"));
}

#[utoipa::path(
    put,
    path = "/api/v1/master/hospitals/{id}",
    params(
        ("id" = Uuid, Path, description = "Hospital ID")
    ),
    request_body = UpdateHospitalRequest,
    responses(
        (status = 200, description = "Hospital updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Hospital not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_hospital(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateHospitalRequest>,
) -> Result<HttpResponse> {
    let hospital_id = path.into_inner();

    // let mut hospital = OrgHospitalRepo { db: &db }
    //     .get_by_id(hospital_id)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    //     .ok_or_else(|| actix_web::error::ErrorNotFound("Hospital not found"))?;

    // // Apply updates
    // if let Some(name) = body.name.clone() { hospital.name = name; }
    // if let Some(status) = body.status.clone() { hospital.status = status; }
    // hospital.updated_at = chrono::Utc::now();

    // OrgHospitalRepo { db: &db }
    //     .update(hospital_id, &hospital)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update hospital"))?;

    return Err(actix_web::error::ErrorNotFound("Hospital not found"));
}

// Facility Management
#[utoipa::path(
    post,
    path = "/api/v1/master/facilities",
    request_body = CreateFacilityRequest,
    responses(
        (status = 201, description = "Facility created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_facility(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateFacilityRequest>,
) -> Result<HttpResponse> {
    let facility_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let facility = OrgFacility {
        facility_id,
        hospital_id: body.hospital_id,
        code: body.code.clone(),
        name: body.name.clone(),
        address_line1: body.address_line1.clone(),
        address_line2: body.address_line2.clone(),
        district: body.district.clone(),
        city: body.city.clone(),
        province: body.province.clone(),
        country: body.country.clone(),
        postal_code: body.postal_code.clone(),
        status: body.status.clone().unwrap_or_else(|| "ACTIVE".to_string()),
        created_at: now,
        created_by: None,
        updated_at: now,
        updated_by: None,
        deleted_at: None,
        deleted_by: None,
    };

    // OrgFacilityRepo { db: &db }
    //     .create(&facility)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create facility"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": facility_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/facilities",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("hospital_id" = Option<Uuid>, Query, description = "Filter by hospital ID"),
        ("code" = Option<String>, Query, description = "Filter by facility code"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of facilities"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_facilities(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<FacilityQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    // let facilities = OrgFacilityRepo { db: &db }
    //     .list_paged(query.hospital_id, query.code.clone(), query.status.clone(), page_size, offset)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;
    let facilities: Vec<OrgFacility> = vec![];

    Ok(HttpResponse::Ok().json(facilities))
}
