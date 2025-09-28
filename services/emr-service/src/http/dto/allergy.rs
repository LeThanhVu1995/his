use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use validator::Validate;
use crate::http::dto::common::ApiResponse;

// Allergy Intolerance DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAllergyRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 255))]
    pub allergen_code: String,

    #[validate(length(min = 1, max = 255))]
    pub allergen_name: String,

    #[validate(length(max = 64))]
    pub category: Option<String>,

    #[validate(length(max = 64))]
    pub severity: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub onset_date: Option<NaiveDate>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateAllergyRequest {
    #[validate(length(max = 255))]
    pub allergen_code: Option<String>,

    #[validate(length(max = 255))]
    pub allergen_name: Option<String>,

    #[validate(length(max = 64))]
    pub category: Option<String>,

    #[validate(length(max = 64))]
    pub severity: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub onset_date: Option<NaiveDate>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergyResponse {
    pub allergy_id: String,
    pub patient_id: String,
    pub encounter_id: String,
    pub allergen_code: String,
    pub allergen_name: String,
    pub category: Option<String>,
    pub severity: Option<String>,
    pub description: Option<String>,
    pub onset_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl AllergyResponse {
    pub fn from_entity(entity: crate::domain::entities::allergy::AllergyIntolerance) -> Self {
        Self {
            allergy_id: entity.allergy_id,
            patient_id: entity.patient_id,
            encounter_id: "".to_string(), // Not available in entity
            allergen_code: entity.substance_code.clone(),
            allergen_name: entity.substance_code, // Use substance_code as name
            category: None, // Not available in entity
            severity: entity.severity_code,
            description: entity.reaction_text,
            onset_date: Some(entity.recorded_at.date_naive()),
            notes: None, // Not available in entity
            created_at: entity.recorded_at,
            updated_at: entity.recorded_at,
            created_by: None, // Not available in entity
        }
    }
}

// Medication Statement DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateMedicationRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 255))]
    pub medication_code: String,

    #[validate(length(min = 1, max = 255))]
    pub medication_name: String,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    #[validate(length(max = 1000))]
    pub dosage: Option<String>,

    #[validate(length(max = 1000))]
    pub frequency: Option<String>,

    #[validate(length(max = 1000))]
    pub route: Option<String>,

    pub start_date: Option<NaiveDate>,

    pub end_date: Option<NaiveDate>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMedicationRequest {
    #[validate(length(max = 255))]
    pub medication_code: Option<String>,

    #[validate(length(max = 255))]
    pub medication_name: Option<String>,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    #[validate(length(max = 1000))]
    pub dosage: Option<String>,

    #[validate(length(max = 1000))]
    pub frequency: Option<String>,

    #[validate(length(max = 1000))]
    pub route: Option<String>,

    pub start_date: Option<NaiveDate>,

    pub end_date: Option<NaiveDate>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationResponse {
    pub medication_id: String,
    pub patient_id: String,
    pub encounter_id: String,
    pub medication_code: String,
    pub medication_name: String,
    pub status: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub route: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl MedicationResponse {
    pub fn from_entity(entity: crate::domain::entities::allergy::MedicationStatement) -> Self {
        Self {
            medication_id: entity.med_stmt_id,
            patient_id: entity.patient_id,
            encounter_id: "".to_string(), // Not available in entity
            medication_code: entity.drug_code,
            medication_name: entity.drug_name,
            status: entity.status,
            dosage: entity.dose_text,
            frequency: entity.frequency_text,
            route: entity.route_code,
            start_date: entity.start_date,
            end_date: entity.end_date,
            notes: None, // Not available in entity
            created_at: chrono::Utc::now(), // Default value
            updated_at: chrono::Utc::now(), // Default value
            created_by: None, // Not available in entity
        }
    }
}

// Query DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListAllergyQuery {
    #[validate(length(max = 64))]
    pub category: Option<String>,

    #[validate(length(max = 64))]
    pub severity: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListMedicationQuery {
    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// From entity implementations
impl From<crate::domain::entities::allergy::AllergyIntolerance> for AllergyResponse {
    fn from(entity: crate::domain::entities::allergy::AllergyIntolerance) -> Self {
        Self {
            allergy_id: entity.allergy_id,
            patient_id: entity.patient_id,
            encounter_id: "".to_string(), // Not available in entity
            allergen_code: entity.substance_code.clone(),
            allergen_name: entity.substance_code, // Use substance_code as name
            category: None, // Not available in entity
            severity: entity.severity_code,
            description: entity.reaction_text,
            onset_date: Some(entity.recorded_at.date_naive()),
            notes: None, // Not available in entity
            created_at: entity.recorded_at,
            updated_at: entity.recorded_at,
            created_by: None, // Not available in entity
        }
    }
}

impl From<crate::domain::entities::allergy::MedicationStatement> for MedicationResponse {
    fn from(entity: crate::domain::entities::allergy::MedicationStatement) -> Self {
        Self {
            medication_id: entity.med_stmt_id,
            patient_id: entity.patient_id,
            encounter_id: "".to_string(), // Not available in entity
            medication_code: entity.drug_code,
            medication_name: entity.drug_name,
            status: entity.status,
            dosage: entity.dose_text,
            frequency: entity.frequency_text,
            route: entity.route_code,
            start_date: entity.start_date,
            end_date: entity.end_date,
            notes: None, // Not available in entity
            created_at: chrono::Utc::now(), // Default value
            updated_at: chrono::Utc::now(), // Default value
            created_by: None, // Not available in entity
        }
    }
}

// Type aliases for API responses
pub type AllergyApiResponse = ApiResponse<AllergyResponse>;
pub type AllergyListResponse = ApiResponse<Vec<AllergyResponse>>;
pub type MedicationApiResponse = ApiResponse<MedicationResponse>;
pub type MedicationListResponse = ApiResponse<Vec<MedicationResponse>>;
