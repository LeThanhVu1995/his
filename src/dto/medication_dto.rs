use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMedicationReq {
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub strength: Option<String>,
    pub form: Option<String>,
    pub route: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateMedicationReq {
    pub name: Option<String>,
    pub strength: Option<String>,
    pub form: Option<String>,
    pub route: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MedQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MedicationRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub strength: Option<String>,
    pub form: Option<String>,
    pub route: Option<String>,
}
