use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use validator::Validate;
use crate::http::dto::common::ApiResponse;

// Patient DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePatientRequest {
    #[validate(length(min = 1, max = 36))]
    pub hospital_id: String,

    #[validate(length(max = 64))]
    pub code: Option<String>,

    #[validate(length(max = 64))]
    pub national_id: Option<String>,

    #[validate(length(min = 1, max = 255))]
    pub full_name: String,

    pub date_of_birth: Option<NaiveDate>,

    #[validate(length(max = 16))]
    pub gender: Option<String>,

    #[validate(length(max = 20))]
    pub phone_number: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(max = 255))]
    pub address_line1: Option<String>,

    #[validate(length(max = 255))]
    pub address_line2: Option<String>,

    #[validate(length(max = 255))]
    pub district: Option<String>,

    #[validate(length(max = 255))]
    pub city: Option<String>,

    #[validate(length(max = 255))]
    pub province: Option<String>,

    #[validate(length(max = 64))]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePatientRequest {
    #[validate(length(max = 64))]
    pub code: Option<String>,

    #[validate(length(max = 64))]
    pub national_id: Option<String>,

    #[validate(length(min = 1, max = 255))]
    pub full_name: Option<String>,

    pub date_of_birth: Option<NaiveDate>,

    #[validate(length(max = 16))]
    pub gender: Option<String>,

    #[validate(length(max = 20))]
    pub phone_number: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(max = 255))]
    pub address_line1: Option<String>,

    #[validate(length(max = 255))]
    pub address_line2: Option<String>,

    #[validate(length(max = 255))]
    pub district: Option<String>,

    #[validate(length(max = 255))]
    pub city: Option<String>,

    #[validate(length(max = 255))]
    pub province: Option<String>,

    #[validate(length(max = 64))]
    pub country: Option<String>,

    #[validate(length(max = 20))]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientResponse {
    pub patient_id: String,
    pub hospital_id: String,
    pub code: Option<String>,
    pub national_id: Option<String>,
    pub full_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<String>,
}

impl PatientResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::Patient) -> Self {
        Self {
            patient_id: entity.patient_id,
            hospital_id: entity.hospital_id,
            code: entity.code,
            national_id: entity.national_id,
            full_name: entity.full_name,
            date_of_birth: entity.date_of_birth,
            gender: entity.gender,
            phone_number: entity.phone_number,
            email: entity.email,
            address_line1: entity.address_line1,
            address_line2: entity.address_line2,
            district: entity.district,
            city: entity.city,
            province: entity.province,
            country: entity.country,
            status: entity.status,
            created_at: entity.created_at,
            created_by: entity.created_by,
            updated_at: entity.updated_at,
            updated_by: entity.updated_by,
        }
    }
}

// Patient Identifier DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePatientIdentifierRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 64))]
    pub system_code: String,

    #[validate(length(min = 1, max = 255))]
    pub value: String,

    pub active: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientIdentifierResponse {
    pub patient_identifier_id: String,
    pub patient_id: String,
    pub system_code: String,
    pub value: String,
    pub active: Option<String>,
}

impl PatientIdentifierResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::PatientIdentifier) -> Self {
        Self {
            patient_identifier_id: entity.patient_identifier_id,
            patient_id: entity.patient_id,
            system_code: entity.system_code,
            value: entity.value,
            active: entity.active,
        }
    }
}

// Patient Contact DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePatientContactRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(max = 64))]
    pub relation_code: Option<String>,

    #[validate(length(max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 20))]
    pub phone_number: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(max = 255))]
    pub address_line1: Option<String>,

    #[validate(length(max = 255))]
    pub address_line2: Option<String>,

    #[validate(length(max = 255))]
    pub city: Option<String>,

    #[validate(length(max = 255))]
    pub province: Option<String>,

    #[validate(length(max = 64))]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientContactResponse {
    pub patient_contact_id: String,
    pub patient_id: String,
    pub relation_code: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub is_primary: Option<String>,
}

impl PatientContactResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::PatientContact) -> Self {
        Self {
            patient_contact_id: entity.patient_contact_id,
            patient_id: entity.patient_id,
            relation_code: entity.relation_code,
            name: entity.name,
            phone_number: entity.phone_number,
            email: entity.email,
            address_line1: entity.address_line1,
            address_line2: entity.address_line2,
            city: entity.city,
            province: None, // Not available in entity
            country: entity.country,
            is_primary: entity.is_primary,
        }
    }
}

// Episode of Care DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateEpisodeRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 64))]
    pub episode_type: String,

    pub start_date: Option<NaiveDate>,

    pub end_date: Option<NaiveDate>,

    #[validate(length(max = 255))]
    pub diagnosis: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeResponse {
    pub episode_id: String,
    pub patient_id: String,
    pub episode_type: String,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub diagnosis: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl EpisodeResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::EpisodeOfCare) -> Self {
        Self {
            episode_id: entity.episode_id,
            patient_id: entity.patient_id,
            episode_type: "episode".to_string(), // Default value
            status: entity.status,
            start_date: Some(entity.start_date.date_naive()),
            end_date: entity.end_date.map(|d| d.date_naive()),
            diagnosis: None, // Not available in entity
            description: entity.reason_text,
            created_at: entity.start_date, // Use start_date as created_at
            updated_at: entity.start_date, // Use start_date as updated_at
            created_by: None, // Not available in entity
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CloseEpisodeRequest {
    pub end_date: NaiveDate,
}

// Search DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchPatientRequest {
    #[validate(length(min = 1, max = 36))]
    pub hospital_id: String,

    #[validate(length(max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 64))]
    pub patient_code: Option<String>,

    #[validate(length(max = 20))]
    pub phone: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// List DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// From entity implementations
impl From<crate::domain::entities::problem::Patient> for PatientResponse {
    fn from(entity: crate::domain::entities::problem::Patient) -> Self {
        Self {
            patient_id: entity.patient_id,
            hospital_id: entity.hospital_id,
            code: entity.code,
            national_id: entity.national_id,
            full_name: entity.full_name,
            date_of_birth: entity.date_of_birth,
            gender: entity.gender,
            phone_number: entity.phone_number,
            email: entity.email,
            address_line1: entity.address_line1,
            address_line2: entity.address_line2,
            district: entity.district,
            city: entity.city,
            province: entity.province,
            country: entity.country,
            status: entity.status,
            created_at: entity.created_at,
            created_by: entity.created_by,
            updated_at: entity.updated_at,
            updated_by: entity.updated_by,
        }
    }
}

impl From<crate::domain::entities::problem::PatientIdentifier> for PatientIdentifierResponse {
    fn from(entity: crate::domain::entities::problem::PatientIdentifier) -> Self {
        Self {
            patient_identifier_id: entity.patient_identifier_id,
            patient_id: entity.patient_id,
            system_code: entity.system_code,
            value: entity.value,
            active: entity.active,
        }
    }
}

impl From<crate::domain::entities::problem::PatientContact> for PatientContactResponse {
    fn from(entity: crate::domain::entities::problem::PatientContact) -> Self {
        Self {
            patient_contact_id: entity.patient_contact_id,
            patient_id: entity.patient_id,
            relation_code: entity.relation_code,
            name: entity.name,
            phone_number: entity.phone_number,
            email: entity.email,
            address_line1: entity.address_line1,
            address_line2: entity.address_line2,
            city: entity.city,
            province: None, // Not available in entity
            country: entity.country,
            is_primary: entity.is_primary,
        }
    }
}

impl From<crate::domain::entities::problem::EpisodeOfCare> for EpisodeResponse {
    fn from(entity: crate::domain::entities::problem::EpisodeOfCare) -> Self {
        Self {
            episode_id: entity.episode_id,
            patient_id: entity.patient_id,
            episode_type: "episode".to_string(), // Default value
            status: entity.status,
            start_date: Some(entity.start_date.date_naive()),
            end_date: entity.end_date.map(|d| d.date_naive()),
            diagnosis: None, // Not available in entity
            description: entity.reason_text,
            created_at: entity.start_date, // Use start_date as created_at
            updated_at: entity.start_date, // Use start_date as updated_at
            created_by: None, // Not available in entity
        }
    }
}

// Type aliases for API responses
pub type PatientApiResponse = ApiResponse<PatientResponse>;
pub type PatientListResponse = ApiResponse<Vec<PatientResponse>>;
pub type PatientIdentifierApiResponse = ApiResponse<PatientIdentifierResponse>;
pub type PatientIdentifierListResponse = ApiResponse<Vec<PatientIdentifierResponse>>;
pub type PatientContactApiResponse = ApiResponse<PatientContactResponse>;
pub type PatientContactListResponse = ApiResponse<Vec<PatientContactResponse>>;
pub type EpisodeApiResponse = ApiResponse<EpisodeResponse>;
pub type EpisodeListResponse = ApiResponse<Vec<EpisodeResponse>>;
