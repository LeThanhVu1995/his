use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Procedure {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub modality: String,
    pub body_part: Option<String>,
    pub contrast: bool,
    pub duration_min: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct RadOrder {
    pub id: Uuid,
    pub order_no: String,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub procedure_id: Uuid,
    pub reason: Option<String>,
    pub priority: String,
    pub status: String,
    pub requested_by: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Study {
    pub id: Uuid,
    pub study_uid: Uuid,
    pub order_id: Uuid,
    pub accession_no: String,
    pub modality: String,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub performer: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Series {
    pub id: Uuid,
    pub study_id: Uuid,
    pub series_no: i32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Image {
    pub id: Uuid,
    pub series_id: Uuid,
    pub instance_no: i32,
    pub sop_uid: Uuid,
    pub storage_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Report {
    pub id: Uuid,
    pub report_no: String,
    pub study_id: Uuid,
    pub status: String,
    pub content: Option<String>,
    pub author: Option<String>,
    pub verified_by: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub finalized_by: Option<String>,
    pub finalized_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
