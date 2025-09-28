use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ImagingReport {
    pub report_id: Uuid,
    pub study_id: Uuid,
    pub report_no: String,
    pub status: String,
    pub report_text: Option<String>,
    pub findings: Option<String>,
    pub impression: Option<String>,
    pub recommendations: Option<String>,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub finalized_by: Option<Uuid>,
    pub finalized_at: Option<DateTime<Utc>>,
    pub amended_by: Option<Uuid>,
    pub amended_at: Option<DateTime<Utc>>,
    pub amendment_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ReportTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub modality: String,
    pub body_part: Option<String>,
    pub template_text: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateReportRequest {
    pub study_id: Uuid,
    pub report_no: String,
    pub report_text: Option<String>,
    pub findings: Option<String>,
    pub impression: Option<String>,
    pub recommendations: Option<String>,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateReportRequest {
    pub status: Option<String>,
    pub report_text: Option<String>,
    pub findings: Option<String>,
    pub impression: Option<String>,
    pub recommendations: Option<String>,
    pub author_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FinalizeReportRequest {
    pub verified_by: Option<Uuid>,
    pub finalized_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReportQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub study_id: Option<Uuid>,
    pub report_no: Option<String>,
    pub status: Option<String>,
    pub author_id: Option<Uuid>,
    pub modality: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReportResponse {
    pub report_id: Uuid,
    pub study_id: Uuid,
    pub report_no: String,
    pub status: String,
    pub report_text: Option<String>,
    pub findings: Option<String>,
    pub impression: Option<String>,
    pub recommendations: Option<String>,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub finalized_by: Option<Uuid>,
    pub finalized_at: Option<DateTime<Utc>>,
    pub amended_by: Option<Uuid>,
    pub amended_at: Option<DateTime<Utc>>,
    pub amendment_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}