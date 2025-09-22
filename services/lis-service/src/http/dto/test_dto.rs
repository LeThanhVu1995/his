use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTestReq {
    pub code: String,
    pub name: String,
    pub specimen_type: String,
    pub unit: Option<String>,
    pub ref_low: Option<f64>,
    pub ref_high: Option<f64>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTestReq {
    pub name: Option<String>,
    pub unit: Option<String>,
    pub ref_low: Option<f64>,
    pub ref_high: Option<f64>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct TestQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LabTestRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub specimen_type: String,
    pub unit: Option<String>,
}
