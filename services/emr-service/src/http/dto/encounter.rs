use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDateTime};
use validator::Validate;
use crate::http::dto::common::ApiResponse;

// Encounter DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateEncounterRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub facility_id: String,

    #[validate(length(min = 1, max = 64))]
    pub encounter_type: String,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub start_time: Option<NaiveDateTime>,

    pub end_time: Option<NaiveDateTime>,

    #[validate(length(max = 1000))]
    pub reason: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateEncounterRequest {
    #[validate(length(max = 64))]
    pub encounter_type: Option<String>,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub start_time: Option<NaiveDateTime>,

    pub end_time: Option<NaiveDateTime>,

    #[validate(length(max = 1000))]
    pub reason: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterResponse {
    pub encounter_id: String,
    pub patient_id: String,
    pub facility_id: String,
    pub encounter_type: String,
    pub status: String,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub reason: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl EncounterResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::Encounter) -> Self {
        Self {
            encounter_id: entity.encounter_id,
            patient_id: entity.patient_id,
            facility_id: entity.facility_id,
            encounter_type: entity.type_code,
            status: entity.status,
            start_time: Some(entity.start_time.naive_utc()),
            end_time: entity.end_time.map(|t| t.naive_utc()),
            reason: None, // Not available in entity
            description: None, // Not available in entity
            created_at: entity.start_time, // Use start_time as created_at
            updated_at: entity.start_time, // Use start_time as updated_at
            created_by: entity.attending_staff_id,
        }
    }
}

// Clinical Note DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateClinicalNoteRequest {
    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 64))]
    pub note_type: String,

    #[validate(length(min = 1, max = 1000))]
    pub content: String,

    #[validate(length(max = 1000))]
    pub title: Option<String>,

    #[validate(length(max = 1000))]
    pub summary: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateClinicalNoteRequest {
    #[validate(length(max = 64))]
    pub note_type: Option<String>,

    #[validate(length(max = 1000))]
    pub content: Option<String>,

    #[validate(length(max = 1000))]
    pub title: Option<String>,

    #[validate(length(max = 1000))]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalNoteResponse {
    pub note_id: String,
    pub encounter_id: String,
    pub note_type: String,
    pub title: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl ClinicalNoteResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::ClinicalNote) -> Self {
        Self {
            note_id: entity.note_id,
            encounter_id: entity.encounter_id,
            note_type: entity.category_code.unwrap_or_else(|| "general".to_string()),
            title: None, // Not available in entity
            content: entity.content_text.unwrap_or_else(|| "".to_string()),
            summary: None, // Not available in entity
            created_at: entity.created_at,
            updated_at: entity.created_at, // Use created_at as updated_at
            created_by: entity.author_staff_id,
        }
    }
}

// Query DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListEncounterQuery {
    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListClinicalNoteQuery {
    #[validate(length(max = 64))]
    pub note_type: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// From entity implementations
impl From<crate::domain::entities::problem::Encounter> for EncounterResponse {
    fn from(entity: crate::domain::entities::problem::Encounter) -> Self {
        Self {
            encounter_id: entity.encounter_id,
            patient_id: entity.patient_id,
            facility_id: entity.facility_id,
            encounter_type: entity.type_code,
            status: entity.status,
            start_time: Some(entity.start_time.naive_utc()),
            end_time: entity.end_time.map(|t| t.naive_utc()),
            reason: None, // Not available in entity
            description: None, // Not available in entity
            created_at: entity.start_time, // Use start_time as created_at
            updated_at: entity.start_time, // Use start_time as updated_at
            created_by: entity.attending_staff_id,
        }
    }
}

impl From<crate::domain::entities::problem::ClinicalNote> for ClinicalNoteResponse {
    fn from(entity: crate::domain::entities::problem::ClinicalNote) -> Self {
        Self {
            note_id: entity.note_id,
            encounter_id: entity.encounter_id,
            note_type: entity.category_code.unwrap_or_else(|| "general".to_string()),
            title: None, // Not available in entity
            content: entity.content_text.unwrap_or_else(|| "".to_string()),
            summary: None, // Not available in entity
            created_at: entity.created_at,
            updated_at: entity.created_at, // Use created_at as updated_at
            created_by: entity.author_staff_id,
        }
    }
}

// Type aliases for API responses
pub type EncounterApiResponse = ApiResponse<EncounterResponse>;
pub type EncounterListResponse = ApiResponse<Vec<EncounterResponse>>;
pub type ClinicalNoteApiResponse = ApiResponse<ClinicalNoteResponse>;
pub type ClinicalNoteListResponse = ApiResponse<Vec<ClinicalNoteResponse>>;
