use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateReportReq {
    pub study_id: Uuid,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EditReportReq {
    pub content: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VerifyReportReq {
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FinalizeReportReq {
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReportQuery {
    pub study_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReportRes {
    pub id: Uuid,
    pub report_no: String,
    pub status: String,
}
