// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
// use utoipa::ToSchema;
// use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrgHospital {
    pub hospital_id: Uuid,
    pub code: String,
    pub name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrgFacility {
    pub facility_id: Uuid,
    pub hospital_id: Uuid,
    pub code: String,
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrgDepartment {
    pub department_id: Uuid,
    pub facility_id: Uuid,
    pub code: String,
    pub name: String,
    pub type_code: Option<String>,
    pub parent_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrgRoom {
    pub room_id: Uuid,
    pub department_id: Uuid,
    pub code: String,
    pub name: Option<String>,
    pub type_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrgBed {
    pub bed_id: Uuid,
    pub room_id: Uuid,
    pub code: String,
    pub name: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// DTOs for API requests
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateHospitalRequest {
    pub code: String,
    pub name: String,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateHospitalRequest {
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateFacilityRequest {
    pub hospital_id: Uuid,
    pub code: String,
    pub name: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateFacilityRequest {
    pub name: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateDepartmentRequest {
    pub facility_id: Uuid,
    pub code: String,
    pub name: String,
    pub type_code: Option<String>,
    pub parent_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateDepartmentRequest {
    pub name: Option<String>,
    pub type_code: Option<String>,
    pub parent_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateRoomRequest {
    pub department_id: Uuid,
    pub code: String,
    pub name: Option<String>,
    pub type_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub type_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateBedRequest {
    pub room_id: Uuid,
    pub code: String,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateBedRequest {
    pub name: Option<String>,
    pub status: Option<String>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct HospitalQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct FacilityQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub hospital_id: Option<Uuid>,
    pub code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct DepartmentQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub facility_id: Option<Uuid>,
    pub code: Option<String>,
    pub type_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RoomQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub department_id: Option<Uuid>,
    pub code: Option<String>,
    pub type_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct BedQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub room_id: Option<Uuid>,
    pub code: Option<String>,
    pub status: Option<String>,
}
