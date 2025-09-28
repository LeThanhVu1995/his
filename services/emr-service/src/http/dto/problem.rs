use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use validator::Validate;
use crate::http::dto::common::ApiResponse;

// Problem List DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProblemRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 255))]
    pub problem_code: String,

    #[validate(length(min = 1, max = 255))]
    pub problem_name: String,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub onset_date: Option<NaiveDate>,

    pub abatement_date: Option<NaiveDate>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,

    #[validate(length(max = 36))]
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProblemRequest {
    #[validate(length(max = 255))]
    pub problem_code: Option<String>,

    #[validate(length(max = 255))]
    pub problem_name: Option<String>,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub onset_date: Option<NaiveDate>,

    pub abatement_date: Option<NaiveDate>,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResolveProblemRequest {
    pub abatement_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemResponse {
    pub problem_id: String,
    pub patient_id: String,
    pub encounter_id: String,
    pub problem_code: String,
    pub problem_name: String,
    pub status: String,
    pub description: Option<String>,
    pub onset_date: Option<NaiveDate>,
    pub abatement_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

impl ProblemResponse {
    pub fn from_entity(entity: crate::domain::entities::problem::ProblemList) -> Self {
        Self {
            problem_id: entity.problem_id,
            patient_id: entity.patient_id,
            encounter_id: "".to_string(), // Not available in entity
            problem_code: entity.code.unwrap_or_else(|| "".to_string()),
            problem_name: entity.description.clone().unwrap_or_else(|| "".to_string()),
            status: entity.status,
            description: entity.description,
            onset_date: entity.onset_date,
            abatement_date: entity.abatement_date,
            notes: None, // Not available in entity
            created_at: chrono::Utc::now(), // Default value
            updated_at: chrono::Utc::now(), // Default value
            created_by: None, // Not available in entity
        }
    }
}

// Query DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListProblemQuery {
    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// From entity implementations
impl From<crate::domain::entities::problem::ProblemList> for ProblemResponse {
    fn from(entity: crate::domain::entities::problem::ProblemList) -> Self {
        Self {
            problem_id: entity.problem_id,
            patient_id: entity.patient_id,
            encounter_id: "".to_string(), // Not available in entity
            problem_code: entity.code.unwrap_or_else(|| "".to_string()),
            problem_name: entity.description.clone().unwrap_or_else(|| "".to_string()),
            status: entity.status,
            description: entity.description,
            onset_date: entity.onset_date,
            abatement_date: entity.abatement_date,
            notes: None, // Not available in entity
            created_at: chrono::Utc::now(), // Default value
            updated_at: chrono::Utc::now(), // Default value
            created_by: None, // Not available in entity
        }
    }
}

// Type aliases for API responses
pub type ProblemApiResponse = ApiResponse<ProblemResponse>;
pub type ProblemListResponse = ApiResponse<Vec<ProblemResponse>>;
