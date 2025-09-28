use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrCase {
    pub or_case_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub scheduled_room_id: Option<Uuid>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: String,
    pub procedure_text: Option<String>,
    pub surgeon_staff_id: Option<Uuid>,
    pub anesthetist_staff_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrChecklist {
    pub checklist_id: Uuid,
    pub or_case_id: Uuid,
    pub phase_code: String,
    pub item_code: String,
    pub completed: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// DTOs for API requests
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrCaseRequest {
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub scheduled_room_id: Option<Uuid>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub procedure_text: Option<String>,
    pub surgeon_staff_id: Option<Uuid>,
    pub anesthetist_staff_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateOrCaseRequest {
    pub scheduled_room_id: Option<Uuid>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub procedure_text: Option<String>,
    pub surgeon_staff_id: Option<Uuid>,
    pub anesthetist_staff_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrChecklistRequest {
    pub or_case_id: Uuid,
    pub phase_code: String,
    pub item_code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateOrChecklistRequest {
    pub completed: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct OrCaseQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<String>,
    pub patient_id: Option<Uuid>,
    pub surgeon_staff_id: Option<Uuid>,
    pub scheduled_room_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct OrChecklistQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub or_case_id: Option<Uuid>,
    pub phase_code: Option<String>,
    pub completed: Option<String>,
}

// Response DTOs with additional info
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrCaseWithChecklist {
    pub or_case_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub scheduled_room_id: Option<Uuid>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: String,
    pub procedure_text: Option<String>,
    pub surgeon_staff_id: Option<Uuid>,
    pub anesthetist_staff_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub checklists: Vec<OrChecklist>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct OrCaseStats {
    pub total: i64,
    pub scheduled: i64,
    pub in_progress: i64,
    pub completed: i64,
    pub cancelled: i64,
}
