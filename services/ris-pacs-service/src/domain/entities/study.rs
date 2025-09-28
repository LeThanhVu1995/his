use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ImagingStudy {
    pub study_id: Uuid,
    pub imaging_order_id: Uuid,
    pub study_uid: String,
    pub accession_no: String,
    pub modality: String,
    pub study_date: NaiveDate,
    pub study_time: Option<NaiveTime>,
    pub study_description: Option<String>,
    pub patient_age: Option<String>,
    pub patient_sex: Option<String>,
    pub referring_physician: Option<String>,
    pub performing_physician: Option<String>,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ImagingSeries {
    pub series_id: Uuid,
    pub study_id: Uuid,
    pub series_uid: String,
    pub series_no: i32,
    pub modality: String,
    pub series_description: Option<String>,
    pub body_part_examined: Option<String>,
    pub protocol_name: Option<String>,
    pub operator_name: Option<String>,
    pub performed_procedure_step_start_date: Option<NaiveDate>,
    pub performed_procedure_step_start_time: Option<NaiveTime>,
    pub number_of_instances: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ImagingInstance {
    pub instance_id: Uuid,
    pub series_id: Uuid,
    pub sop_instance_uid: String,
    pub instance_no: i32,
    pub content_date: Option<NaiveDate>,
    pub content_time: Option<NaiveTime>,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub transfer_syntax_uid: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateStudyRequest {
    pub imaging_order_id: Uuid,
    pub study_uid: String,
    pub accession_no: String,
    pub modality: String,
    pub study_date: NaiveDate,
    pub study_time: Option<NaiveTime>,
    pub study_description: Option<String>,
    pub patient_age: Option<String>,
    pub patient_sex: Option<String>,
    pub referring_physician: Option<String>,
    pub performing_physician: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateStudyRequest {
    pub status: Option<String>,
    pub study_description: Option<String>,
    pub performing_physician: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudyQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub imaging_order_id: Option<Uuid>,
    pub study_uid: Option<String>,
    pub accession_no: Option<String>,
    pub modality: Option<String>,
    pub status: Option<String>,
    pub study_date_from: Option<NaiveDate>,
    pub study_date_to: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StudyResponse {
    pub study_id: Uuid,
    pub imaging_order_id: Uuid,
    pub study_uid: String,
    pub accession_no: String,
    pub modality: String,
    pub study_date: NaiveDate,
    pub study_time: Option<NaiveTime>,
    pub study_description: Option<String>,
    pub patient_age: Option<String>,
    pub patient_sex: Option<String>,
    pub referring_physician: Option<String>,
    pub performing_physician: Option<String>,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}