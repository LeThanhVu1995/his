use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateResultReq {
    pub specimen_id: Uuid,
    pub test_id: Uuid,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EnterResultReq {
    pub values: Vec<ResultValueReq>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResultValueReq {
    pub analyte_code: String,
    pub analyte_name: String,
    pub value_num: Option<f64>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub ref_low: Option<f64>,
    pub ref_high: Option<f64>,
    pub flag: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ResultQuery {
    pub specimen_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LabResultRes {
    pub id: Uuid,
    pub result_no: String,
    pub status: String,
}
