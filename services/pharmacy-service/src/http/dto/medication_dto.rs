use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMedicationReq {
    pub code: String,
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

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct MedicationQuery {
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

