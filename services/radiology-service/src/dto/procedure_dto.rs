use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProcedureReq {
    pub code: String,
    pub name: String,
    pub modality: String,
    pub body_part: Option<String>,
    pub contrast: Option<bool>,
    pub duration_min: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProcedureReq {
    pub name: Option<String>,
    pub body_part: Option<String>,
    pub contrast: Option<bool>,
    pub duration_min: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProcQuery {
    pub q: Option<String>,
    pub modality: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProcedureRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub modality: String,
}
