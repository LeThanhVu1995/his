use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDateTime};
use validator::Validate;
use crate::http::dto::common::ApiResponse;

// Vital Sign Record DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateVitalSignRecordRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 64))]
    pub record_type: String,

    pub recorded_at: Option<NaiveDateTime>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateVitalSignRecordRequest {
    #[validate(length(max = 64))]
    pub record_type: Option<String>,

    pub recorded_at: Option<NaiveDateTime>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSignRecordResponse {
    pub record_id: String,
    pub patient_id: String,
    pub encounter_id: String,
    pub record_type: String,
    pub recorded_at: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl VitalSignRecordResponse {
    pub fn from_entity(entity: crate::domain::entities::vital::VitalSignRecord) -> Self {
        Self::from(entity)
    }
}

// Vital Sign Item DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateVitalSignItemRequest {
    #[validate(length(min = 1, max = 36))]
    pub record_id: String,

    #[validate(length(min = 1, max = 64))]
    pub vital_type: String,

    #[validate(length(min = 1, max = 255))]
    pub value: String,

    #[validate(length(max = 64))]
    pub unit: Option<String>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateVitalSignItemRequest {
    #[validate(length(max = 64))]
    pub vital_type: Option<String>,

    #[validate(length(max = 255))]
    pub value: Option<String>,

    #[validate(length(max = 64))]
    pub unit: Option<String>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSignItemResponse {
    pub item_id: String,
    pub record_id: String,
    pub vital_type: String,
    pub value: String,
    pub unit: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VitalSignItemResponse {
    pub fn from_entity(entity: crate::domain::entities::vital::VitalSignItem) -> Self {
        Self::from(entity)
    }
}

// Observation DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateObservationRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 64))]
    pub observation_type: String,

    #[validate(length(min = 1, max = 255))]
    pub value: String,

    #[validate(length(max = 64))]
    pub unit: Option<String>,

    #[validate(length(max = 1000))]
    pub interpretation: Option<String>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,

    pub observed_at: Option<NaiveDateTime>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateObservationRequest {
    #[validate(length(max = 64))]
    pub observation_type: Option<String>,

    #[validate(length(max = 255))]
    pub value: Option<String>,

    #[validate(length(max = 64))]
    pub unit: Option<String>,

    #[validate(length(max = 1000))]
    pub interpretation: Option<String>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,

    pub observed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationResponse {
    pub observation_id: String,
    pub patient_id: String,
    pub encounter_id: String,
    pub observation_type: String,
    pub value: String,
    pub unit: Option<String>,
    pub interpretation: Option<String>,
    pub notes: Option<String>,
    pub observed_at: Option<NaiveDateTime>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl ObservationResponse {
    pub fn from_entity(entity: crate::domain::entities::vital::Observation) -> Self {
        Self::from(entity)
    }
}

// Query DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListVitalSignQuery {
    #[validate(length(max = 64))]
    pub record_type: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListObservationQuery {
    #[validate(length(max = 64))]
    pub observation_type: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// From entity implementations
impl From<crate::domain::entities::vital::VitalSignRecord> for VitalSignRecordResponse {
    fn from(entity: crate::domain::entities::vital::VitalSignRecord) -> Self {
        Self {
            record_id: entity.vs_id,
            patient_id: entity.patient_id,
            encounter_id: entity.encounter_id,
            record_type: "vital_signs".to_string(), // Default value
            recorded_at: Some(entity.measured_at.naive_utc()),
            notes: entity.note,
            created_at: entity.measured_at,
            updated_at: entity.measured_at,
            created_by: entity.recorder_staff_id,
        }
    }
}

impl From<crate::domain::entities::vital::VitalSignItem> for VitalSignItemResponse {
    fn from(entity: crate::domain::entities::vital::VitalSignItem) -> Self {
        Self {
            item_id: entity.vs_item_id,
            record_id: entity.vs_id,
            vital_type: entity.code,
            value: entity.value_text.unwrap_or_else(|| entity.value_num.map(|v| v.to_string()).unwrap_or_else(|| "".to_string())),
            unit: entity.unit,
            notes: None, // Not available in entity
            created_at: chrono::Utc::now(), // Default value
            updated_at: chrono::Utc::now(), // Default value
        }
    }
}

impl From<crate::domain::entities::vital::Observation> for ObservationResponse {
    fn from(entity: crate::domain::entities::vital::Observation) -> Self {
        Self {
            observation_id: entity.obs_id,
            patient_id: entity.patient_id,
            encounter_id: entity.encounter_id,
            observation_type: entity.code,
            value: entity.value_text.unwrap_or_else(|| entity.value_num.map(|v| v.to_string()).unwrap_or_else(|| "".to_string())),
            unit: entity.unit,
            interpretation: None, // Not available in entity
            notes: None, // Not available in entity
            observed_at: Some(entity.taken_at.naive_utc()),
            created_at: entity.taken_at,
            updated_at: entity.taken_at,
            created_by: entity.performer_staff_id,
        }
    }
}

// Type aliases for API responses
pub type VitalSignRecordApiResponse = ApiResponse<VitalSignRecordResponse>;
pub type VitalSignRecordListResponse = ApiResponse<Vec<VitalSignRecordResponse>>;
pub type VitalSignItemApiResponse = ApiResponse<VitalSignItemResponse>;
pub type VitalSignItemListResponse = ApiResponse<Vec<VitalSignItemResponse>>;
pub type ObservationApiResponse = ApiResponse<ObservationResponse>;
pub type ObservationListResponse = ApiResponse<Vec<ObservationResponse>>;
